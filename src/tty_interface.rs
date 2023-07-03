use crate::core::{GameStatus, Status, Error, BASE_CHAR};
use std::fs;
use std::iter::zip;
use console;
use crate::interface::Interface;


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

pub struct TTYInterface {
    target: String,
}

impl TTYInterface {
    pub fn new() -> Self {
        Self {
            target: String::new(),
        }
    }
}

impl Interface for TTYInterface {
    fn start(&mut self, target: &str) {
        self.target = target.to_string();
        match fs::read_to_string(WORDLE_RES) {
            Ok(content) => {
                println!("{}", console::style(content).bold().blue());
            },
            Err(error) => {
                println!("{}", error);
                println!("{}", console::style("WORDLE").bold().italic().cyan());
            }
        }
    }

    fn difficult_message(&mut self) {
        println!("{}",console::style("Invalid in difficult mode.").bold().yellow());
    }

    fn guess(&mut self, word: &str, game: &mut GameStatus) {
        match game.guess(&word) {
            Ok(result) => {
                for (c, s) in zip(word.chars(), result) {
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
    }

    fn end(&mut self, game: &GameStatus) {
        if game.end {
            println!("{} in {}",console::style("CORRECT!").bold().italic().green(), game.trials);
        } else {
            println!("{}\nThe answer is {}",console::style("FAILED").bold().red().dim(), self.target);
        }
    }
}