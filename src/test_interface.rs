use std::io;
use crate::core::GameStatus;

const MAX_TRIAL: u32 = 6;

pub fn test_mode(target: &str) {
    let mut game = GameStatus::new(target);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    while game.trials < MAX_TRIAL {
        match game.guess(&input) {
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
        if game.end {
            println!("CORRECT {}", game.trials);
            return;
        }
    }
    println!("FAILED {}", target);
}