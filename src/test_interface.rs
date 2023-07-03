use crate::core::GameStatus;
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

    fn guess(&mut self, word: &str, game: &mut GameStatus) {
        match game.guess(&word) {
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
    }

    fn end(&mut self, game: &GameStatus) {
        if game.end {
            println!("CORRECT {}", game.trials);
        } else {
            println!("FAILED {}", self.target);
        }
    }
}