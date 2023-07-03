mod builtin_words;
mod core;
mod test_interface;
mod tty_interface;
mod interface;


use std::io;
use crate::builtin_words::FINAL;
use clap::Parser;
use rand::seq::SliceRandom;
use crate::core::GameStatus;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Wordle is a game where you guess a 5-letter word.")]
struct Args {
    /// Target word to guess, overrides random mode
    #[arg(short, long)]
    word: Option<String>,

    /// Whether use a random target
    #[arg(short, long, default_value_t = false)]
    random: bool,

    /// Whether use difficult mode
    #[arg(short = Some('D'), long, default_value_t = false)]
    difficult: bool,
}


fn game_round(target: &String, args: &Args) -> GameStatus {
    let is_tty = atty::is(atty::Stream::Stdout);
    assert!(FINAL.contains(&target.trim().to_lowercase().as_str()));
    let target = target.trim().to_uppercase();
    let target = target.as_str();
    if is_tty {
        interface::run(tty_interface::TTYInterface::new(), target, args.difficult)
    } else {
        interface::run(test_interface::TestInterface::new(), target, args.difficult)
    }
}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.word == None {
        let mut used_words: Vec<&str> = Vec::new();
        loop {
            let result = if args.random {
                let mut rng = rand::thread_rng();
                let mut target = FINAL.choose(&mut rng).unwrap();
                while used_words.contains(target) {
                    target = FINAL.choose(&mut rng).unwrap();
                }
                used_words.push(*target);
                game_round(&target.to_string(), &args)
            } else {
                let mut target = String::new();
                io::stdin().read_line(&mut target)?;
                game_round(&target, &args)
            };

        }
    } else {
        game_round(&args.word.clone().unwrap(), &args);
    }
    Ok(())
}
