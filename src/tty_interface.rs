use crate::core::{GameStatus, Status, Error, BASE_CHAR};
use std::io;
use std::fs;
use std::iter::zip;
use console;


const MAX_TRIAL: u32 = 6;
const WORDLE_RES: &str = "res/wordle.txt";

fn to_colorful_char(c: String, status: &Status) -> console::StyledObject<String> {
    match status {
        Status::Green => console::style(c).green(),
        Status::Yellow => console::style(c).yellow(),
        Status::Red => console::style(c).red(),
        Status::Grey => console::style(c),
    }
}


pub fn tty_mode(target: &str) {
    match fs::read_to_string(WORDLE_RES) {
        Ok(content) => {
            println!("{}", console::style(content).bold().blue());
        },
        Err(error) => {
            println!("{}", error);
            println!("{}", console::style("WORDLE").bold().italic().cyan());
        }
    }
    let mut game = GameStatus::new(target);
    while game.trials < MAX_TRIAL {
        let mut input = String::new();
        print!("{} trails left: ", MAX_TRIAL - game.trials);
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();
        match game.guess(&input) {
            Ok(result) => {
                for (c, s) in zip(input.chars(), result) {
                    print!("{}", to_colorful_char(c.to_string(), &s));
                }
                print!(" ");
                for (i, s) in game.keyboard.iter().enumerate() {
                    print!("{}", to_colorful_char(char::from_u32((i + BASE_CHAR as usize) as u32).unwrap().to_string(), s));
                }
                println!();
            }
            Err(Error::InvalidWord) => {
                println!("{}",console::style("Not a word.").bold().yellow());
            },
            Err(Error::InvalidWordLength) => {
                println!("{}",console::style("Invalid length.").bold().yellow());
            }
        }
        if game.end {
            println!("{} in {}",console::style("CORRECT!").bold().italic().green(), game.trials);
            return;
        }
    }
    println!("{}\nThe answer is {}",console::style("FAILED.").bold().red().dim(), target);
}
