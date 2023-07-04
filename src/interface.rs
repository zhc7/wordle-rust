use std::io;

use crate::core::GameStatus;

pub const MAX_TRIAL: u32 = 6;

pub trait Interface {
    fn start(&mut self, target: &str);
    fn difficult_message(&mut self);
    fn guess(&mut self, word: &str, game: &mut GameStatus);
    fn end(&mut self, game: &GameStatus);
    fn print_stats(&mut self, top_words: &Vec<(&String, &u32)>, wins: u32, total: u32, trials: u32);
}


pub fn run<'a>(
    interface: &mut Box<dyn Interface>,
    target: &str,
    difficult: bool,
    acceptable_set: &'a Vec<String>,
) -> (GameStatus<'a>, Vec<String>, bool) {
    interface.start(target);
    let mut game = GameStatus::new(target, acceptable_set);
    let mut guesses: Vec<String> = Vec::new();
    while game.trials < MAX_TRIAL {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();
        guesses.push(input.clone());
        if difficult {
            if let Ok(b) = game.check(&input) {
                if !b {
                    interface.difficult_message();
                    continue;
                }
            }
        }
        interface.guess(&input.trim(), &mut game);
        if game.end {
            break;
        }
    }
    interface.end(&mut game);
    let win = game.end;
    (game, guesses, win)
}