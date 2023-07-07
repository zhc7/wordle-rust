use std::collections::HashMap;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use crate::core::{GameStatus};
use crate::builtin_words::{ACCEPTABLE, FINAL};
use rand::seq::SliceRandom;

mod core;
mod builtin_words;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hello world!");
}

#[wasm_bindgen]
pub struct Runner {
    game: GameStatus<'static>,
    guess_count: HashMap<String, u32>,
    wins: u32,
    total: u32,
    trials: u32,
    top_words: Vec<(String, u32)>,
    tmp_ptr: u32,
}

lazy_static!(
    static ref ACC: Vec<String> = ACCEPTABLE.iter().map(|s| s.to_uppercase()).collect();
);

#[wasm_bindgen]
impl Runner {
    pub fn new() -> Self {
        Self {
            game: GameStatus::new("xxxxx", &ACC),
            guess_count: HashMap::new(),
            wins: 0,
            total: 0,
            trials: 0,
            top_words: Vec::new(),
            tmp_ptr: 0,
        }
    }

    pub fn start(&mut self) {
        let target = FINAL.choose(&mut rand::thread_rng()).unwrap();
        self.game = GameStatus::new(target, &ACC);
    }

    pub fn check(&self, word: String) -> u32 {
        match self.game.check(&word) {
            Ok(true) => 1,
            Ok(false) => 2,
            _ => 0,
        }
    }

    pub fn guess(&mut self, word: String) -> String {
        match self.game.guess(&word) {
            Ok(result) => {
                let mut result_str = String::new();
                for s in result {
                    result_str += s.to_str();
                }
                *self.guess_count.entry(word).or_insert(0) += 1;
                result_str
            }
            Err(_) => {
                "Not a word".to_string()
            }
        }
    }

    pub fn nums(&self) -> Vec<u32> {
        vec![self.wins, self.total, self.trials]
    }

    pub fn end(&mut self) {
        self.trials += 1;
        if self.game.end {
            self.wins += 1;
        }
        self.total += 1;
    }

    pub fn top_words(&mut self) {
        self.top_words = self.guess_count.iter().map(|(k, v)| (k.clone(), *v)).collect();
        self.top_words.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        self.tmp_ptr = 0;
    }

    pub fn top_word(&mut self) -> String {
        self.tmp_ptr += 1;
        self.top_words[(self.tmp_ptr - 1) as usize].0.clone()
    }

    pub fn top_word_length(&self) -> usize {
        self.top_words.len()
    }

    pub fn top_word_counts(&self) -> Vec<u32> {
        self.top_words.iter().map(|(_, v)| *v).collect()
    }

    pub fn status(&self) -> u32 {
        if self.game.end {
            1
        } else if self.game.trials >= 6 {
            2
        } else {
            0
        }
    }

    pub fn answer(&self) -> String {
        self.game.target.iter().collect()
    }
}