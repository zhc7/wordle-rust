use std::collections::HashMap;
use std::io;

use clap::Parser;
use rand::seq::SliceRandom;

use crate::builtin_words::FINAL;
use crate::core::GameStatus;
use crate::interface::{Interface, run};

mod builtin_words;
mod core;
mod test_interface;
mod tty_interface;
mod interface;


#[derive(Parser, Debug)]
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
}


fn game_round(target: &String, args: &Args, interface: &mut Box<dyn Interface>) -> (GameStatus, Vec<String>, bool) {
    assert!(FINAL.contains(&target.trim().to_lowercase().as_str()));
    let target = target.trim().to_uppercase();
    let target = target.as_str();
    run(interface, target, args.difficult)
}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut interface: Box<dyn Interface>;
    if atty::is(atty::Stream::Stdout) && false {
        interface = Box::new(tty_interface::TTYInterface::new());
    } else {
        interface = Box::new(test_interface::TestInterface::new());
    }
    let args = Args::parse();
    let mut day = args.day;
    if args.word == None {
        // statistics
        let mut used_words: Vec<&str> = Vec::new();
        let mut guess_count: HashMap<String, u32> = HashMap::new();
        let mut wins: u32 = 0;
        let mut total: u32 = 0;
        let mut trials: u32 = 0;

        // random init
        let v_tmp = FINAL
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();
        let mut dict: Vec<&String> = v_tmp.iter().collect();
        if args.random {
            if let Some(seed) = args.seed {
                let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
                dict.shuffle(&mut rng);
            } else {
                dict.shuffle(&mut rand::thread_rng());
            }
        }

        // multiple rounds loop
        loop {
            let (game, guesses, win) = if args.random {
                let target = dict[day - 1];
                used_words.push(target);
                game_round(&target, &args, &mut interface)
            } else {
                let mut target = String::new();
                io::stdin().read_line(&mut target)?;
                game_round(&target, &args, &mut interface)
            };

            // statistics
            if args.stats {
                wins += win as u32;
                total += 1;
                if win {
                    trials += game.trials;
                }
                for guess in guesses {
                    *guess_count.entry(guess).or_insert(0) += 1;
                }
                // print top five words
                let mut top_five: Vec<(&String, &u32)> = guess_count.iter().collect();
                top_five.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
                interface.print_stats(&top_five, wins, total, trials);
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
        game_round(&args.word.clone().unwrap(), &args, &mut interface);
    }
    Ok(())
}
