use std::collections::HashMap;

use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::builtin_words::{ACCEPTABLE, FINAL};
use crate::core::GameStatus;

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
    local_storage: web_sys::Storage,
}

lazy_static!(
    static ref ACC: Vec<String> = ACCEPTABLE.iter().map(|s| s.to_uppercase()).collect();
);

#[wasm_bindgen]
impl Runner {
    pub fn new() -> Self {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let guess_count: HashMap<String, u32> = if let Some(s) = local_storage.get_item("guess_count").unwrap() {
            s.trim()
                .split(",")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut iter = s.split(":");
                    (iter.next().unwrap().to_string(), iter.next().unwrap().parse().unwrap())
                }).collect()
        } else {
            HashMap::new()
        };
        Self {
            game: GameStatus::new("xxxxx", &ACC),
            guess_count,
            wins: local_storage.get_item("wins_").unwrap().unwrap_or("0".to_string()).parse().unwrap(),
            total: local_storage.get_item("total_").unwrap().unwrap_or("0".to_string()).parse().unwrap(),
            trials: local_storage.get_item("trials_").unwrap().unwrap_or("0".to_string()).parse().unwrap(),
            top_words: Vec::new(),
            tmp_ptr: 0,
            local_storage,
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
        self.local_storage.set_item("wins_", &self.wins.to_string()).unwrap();
        self.local_storage.set_item("total_", &self.total.to_string()).unwrap();
        self.local_storage.set_item("trials_", &self.trials.to_string()).unwrap();
        let mut guess_count_str = String::new();
        for (k, v) in self.guess_count.iter() {
            guess_count_str += &format!("{0}:{1},", k, v);
        }
        self.local_storage.set_item("guess_count", &guess_count_str).unwrap();
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