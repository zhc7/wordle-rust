use crate::core::{Error, GameStatus, Status};
use crate::interface::Interface;

pub struct TestInterface {
    target: String,
}

impl TestInterface {
    pub fn new() -> Self {
        Self {
            target: String::new(),
        }
    }
}

impl Interface for TestInterface {
    fn start(&mut self, target: &str) {
        self.target = target.to_string();
    }

    fn difficult_message(&mut self) {
        println!("INVALID");
    }

    fn guess(&mut self, word: &str, game: &mut GameStatus) -> Result<[Status; 5], Error> {
        let r = game.guess(&word);
        match r {
            Ok(result) => {
                for s in result {
                    print!("{}", s.to_str());
                }
                print!(" ");
                for s in game.keyboard {
                    print!("{}", s.to_str());
                }
                println!();
            }
            Err(_) => {
                println!("INVALID");
            }
        }
        r
    }

    fn end(&mut self, game: &GameStatus) {
        if game.end {
            println!("CORRECT {}", game.trials);
        } else {
            println!("FAILED {}", self.target);
        }
    }

    fn print_stats(&mut self, top_words: &Vec<(&String, &u32)>, wins: u32, total: u32, trials: u32) {
        println!("{} {} {:.2}", wins, total - wins, if wins != 0 {trials as f64 / wins as f64} else {0.0});
        let mut out = String::new();
        for (i, (word, count)) in top_words.iter().enumerate() {
            if i >= 5 {
                break;
            }
            out += &format!("{} {} ", word, count);
        }
        println!("{}", out.trim());
    }
}