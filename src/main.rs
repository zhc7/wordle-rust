use std::collections::HashMap;
use std::io;

use clap::Parser;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::builtin_words::{ACCEPTABLE, FINAL};
use crate::core::GameStatus;
use crate::interface::{Interface, run};

mod builtin_words;
mod core;
mod test_interface;
mod tty_interface;
mod interface;


#[derive(Parser, Debug, Deserialize)]
#[command(author, version, about, long_about = "Wordle is a game where you guess a 5-letter word.")]
struct Args {
    /// Target word to guess, overrides random mode
    #[arg(short, long)]
    word: Option<String>,

    /// use a random target
    #[arg(short, long, default_value_t = false)]
    random: bool,

    /// use difficult mode
    #[arg(short = Some('D'), long, default_value_t = false)]
    difficult: bool,

    /// print status
    #[arg(short = Some('t'), long, default_value_t = false)]
    stats: bool,

    /// rounds when starting
    #[arg(short, long, default_value_t = 1)]
    day: usize,

    /// random seed
    #[arg(short, long)]
    seed: Option<u64>,

    /// use custom target word set
    #[arg(short, long)]
    final_set: Option<String>,

    /// use custom dictionary
    #[arg(short, long)]
    acceptable_set: Option<String>,

    /// load and save game state from and to JSON file
    #[arg(short = Some('S'), long)]
    state: Option<String>,

    /// load arguments from JSON file
    #[arg(short, long)]
    config: Option<String>,
}

impl Args {
    fn merge(self, other: Args) -> Self {
        Self {
            word: self.word.or(other.word),
            random: self.random || other.random,
            difficult: self.difficult || other.difficult,
            stats: self.stats || other.stats,
            day: if self.day == 1 { other.day } else { self.day },
            seed: self.seed.or(other.seed),
            final_set: self.final_set.or(other.final_set),
            acceptable_set: self.acceptable_set.or(other.acceptable_set),
            state: self.state.or(other.state),
            config: self.config.or(other.config),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct SingleGameState {
    answer: String,
    guesses: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GameState {
    total_rounds: u32,
    games: Vec<SingleGameState>,
}


fn game_round<'a>(
    target: &String,
    args: &Args,
    interface: &mut Box<dyn Interface>,
    acceptable_set: &'a Vec<String>,
) -> (GameStatus<'a>, Vec<String>, bool) {
    assert!(FINAL.contains(&target.trim().to_lowercase().as_str()));
    let target = target.trim().to_uppercase();
    let target = target.as_str();
    run(interface, target, args.difficult, acceptable_set)
}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // interface choosing
    let mut interface: Box<dyn Interface>;
    if atty::is(atty::Stream::Stdout) {
        interface = Box::new(tty_interface::TTYInterface::new());
    } else {
        interface = Box::new(test_interface::TestInterface::new());
    }

    // arg parsing
    let cmd_args = Args::parse();
    let args = if let Some(path) = &cmd_args.config {
        let config: Args = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        cmd_args.merge(config)
    } else {
        cmd_args
    };
    let mut day = args.day;
    let final_set: Vec<String> = if let Some(path) = &args.final_set {
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    } else {
        FINAL.iter().map(|s| String::from(*s)).collect()
    };
    // dataset operation
    let mut final_set: Vec<String> = final_set
        .iter()
        .map(|s| s.to_uppercase())
        .collect();
    final_set.sort();
    let acceptable_set: Vec<String> = if let Some(path) = &args.acceptable_set {
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    } else {
        ACCEPTABLE.iter().map(|s| String::from(*s)).collect()
    };
    let mut acceptable_set: Vec<String> = acceptable_set
        .iter()
        .map(|s| s.to_uppercase())
        .collect();
    acceptable_set.sort();
    // check acceptable >= final
    let mut p = 0;
    for s in &final_set {
        while p < acceptable_set.len() && &acceptable_set[p] != s { p += 1; }
        assert!(p < acceptable_set.len(), "{} not contained in acceptable set", s);
    }


    if args.word == None {
        // statistics
        let mut guess_count: HashMap<String, u32> = HashMap::new();
        let mut wins: u32 = 0;
        let mut total: u32 = 0;
        let mut trials: u32 = 0;

        // load state
        let mut states: Vec<SingleGameState> = vec![];
        if let Some(path) = &args.state {
            let loaded_states: GameState = serde_json::from_str(&std::fs::read_to_string(path)?)?;
            wins = loaded_states.games
                .iter()
                .filter(|g| g.guesses.last() == Some(&g.answer))
                .count() as u32;
            total = loaded_states.total_rounds;
            trials = loaded_states.games
                .iter()
                .map(|g| g.guesses.len() as u32)
                .sum();
            for game in loaded_states.games {
                for guess in game.guesses.clone() {
                    *guess_count
                        .entry(guess)
                        .or_insert(0) += 1;
                }
                states.push(game);
            }
        }

        // random init
        if args.random {
            if let Some(seed) = args.seed {
                let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
                final_set.shuffle(&mut rng);
            } else {
                final_set.shuffle(&mut rand::thread_rng());
            }
        }

        // multiple rounds loop
        loop {
            let target: String = if args.random {
                final_set[day - 1].clone()
            } else {
                let mut target = String::new();
                io::stdin().read_line(&mut target)?;
                target.trim().to_string()
            };
            let (game, guesses, win) =
                game_round(&target, &args, &mut interface, &acceptable_set);

            // statistics
            if args.stats {
                wins += win as u32;
                total += 1;
                if win {
                    trials += game.trials;
                }
                for guess in guesses.clone() {
                    *guess_count.entry(guess).or_insert(0) += 1;
                }
                // print top five words
                let mut top_five: Vec<(&String, &u32)> = guess_count.iter().collect();
                top_five
                    .sort_by(|a, b|
                        b.1.cmp(a.1).then(a.0.cmp(b.0)));
                interface.print_stats(&top_five, wins, total, trials);
            }

            // save state
            if let Some(path) = &args.state {
                states.push(SingleGameState {
                    answer: target.clone(),
                    guesses: guesses.iter().map(|s| s.to_string()).collect(),
                });
                let json = serde_json::to_string(&GameState {
                    total_rounds: total,
                    games: states.clone(),
                });
                if let Ok(json) = json {
                    std::fs::write(path, json)?;
                }
            }

            // whether next round
            let mut next = String::new();
            match io::stdin().read_line(&mut next) {
                Ok(0) => break,
                Ok(_) => {
                    if next.trim() == "N" {
                        break;
                    }
                }
                _ => {}
            }
            day += 1;
        }
    } else {
        assert!(args.seed.is_none());
        assert_eq!(args.day, 1);
        game_round(&args.word.clone().unwrap(), &args, &mut interface, &acceptable_set);
    }
    Ok(())
}
