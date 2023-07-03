use std::io;
use crate::core::GameStatus;

const MAX_TRIAL: u32 = 6;

pub trait Interface {
    fn start(&mut self, target: &str);
    fn difficult_message(&mut self);
    fn guess(&mut self, word: &str, game: &mut GameStatus);
    fn end(&mut self, game: &GameStatus);
}

pub fn run<T: Interface>(mut interface: T, target: &str, difficult: bool) -> GameStatus {
    interface.start(target);
    let mut game = GameStatus::new(target);
    while game.trials < MAX_TRIAL {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if difficult {
            if let Ok(b) = game.check(&input) {
                if !b {
                    interface.difficult_message();
                    continue;
                }
            }
        }
        interface.guess(&input.trim(), &mut game);
    }
    interface.end(&mut game);
    game
}