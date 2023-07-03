mod builtin_words;
mod core;
mod test_interface;
mod tty_interface;
mod interface;


use std::collections::HashMap;
use std::io;
use crate::builtin_words::FINAL;
use clap::Parser;
use rand::seq::SliceRandom;
use crate::core::GameStatus;
use crate::interface::{Interface, run};

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
    if atty::is(atty::Stream::Stdout) {
        interface = Box::new(tty_interface::TTYInterface::new());
    } else {
        interface = Box::new(test_interface::TestInterface::new());
    }
    let args = Args::parse();
    if args.word == None {
        let mut used_words: Vec<&str> = Vec::new();
        let mut guess_count: HashMap<String, u32> = HashMap::new();
        let mut wins: u32 = 0;
        let mut total: u32 = 0;
        let mut trials: u32 = 0;
        loop {
            let (game, guesses, win) = if args.random {
                let mut rng = rand::thread_rng();
                let mut target = FINAL.choose(&mut rng).unwrap();
                while used_words.contains(target) {
                    target = FINAL.choose(&mut rng).unwrap();
                }
                used_words.push(*target);
                game_round(&target.to_string(), &args, &mut interface)
            } else {
                let mut target = String::new();
                io::stdin().read_line(&mut target)?;
                game_round(&target, &args, &mut interface)
            };
            for guess in guesses {
                *guess_count.entry(guess).or_insert(0) += 1;
            }
            wins += win as u32;
            total += 1;
            trials += game.trials;
            if args.stats {
                // print top five words
                let mut top_five: Vec<(&String, &u32)> = guess_count.iter().collect();
                top_five.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
                interface.print_stats(&top_five.iter().map(|(word, _)| String::from(*word)).collect(), wins, total, trials);
            }
            let mut next = String::new();
            match io::stdin().read_line(&mut next) {
                Ok(0) => break,
                Ok(_) => {
                    if next == "n" {
                        break;
                    }
                },
                _ => {}
            }
        }
    } else {
        game_round(&args.word.clone().unwrap(), &args, &mut interface);
    }
    Ok(())
}
