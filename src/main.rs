mod builtin_words;
mod core;
mod test_interface;
mod tty_interface;


use std::io;
use crate::builtin_words::FINAL;
use clap::Parser;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Wordle is a game where you guess a 5-letter word.")]
struct Args {
    // Target word to guess, overrides random mode
    #[arg(short, long, default_value = "")]
    word: String,

    // Whether use a random target
    #[arg(short, long, default_value_t = false)]
    random: bool,

    // Whether use difficult mode
    #[arg(short, long, default_value_t = false)]
    difficult: bool,
}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = true; // atty::is(atty::Stream::Stdout);
    let args = Args::parse();
    let target = if args.word.is_empty() {
        if args.random {
            let mut rng = rand::thread_rng();
            let target = FINAL.choose(&mut rng).unwrap();
            target.to_string()
        } else {
            let mut target = String::new();
            io::stdin().read_line(&mut target)?;
            target
        }
    } else { args.word };
    assert!(FINAL.contains(&target.trim().to_lowercase().as_str()));
    let target = target.trim().to_uppercase();
    let target = target.as_str();
    if is_tty {
        tty_interface::tty_mode(target, args.difficult);
    } else {
        test_interface::test_mode(target, args.difficult);
    }
    Ok(())
}
