use std::collections::HashMap;
use lazy_static::lazy_static;
use builtin_words::*;
use crate::builtin_words::ACCEPTABLE;

const WORD_SIZE: usize = 5;
const ALPHABET_SIZE: usize = 26;
const BASE_CHAR: char = 'A';


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Green,
    Yellow,
    Red,
    Grey,
}

impl Status {
    fn to_str(&self) -> &'static str {
        match self {
            Status::Green => "G",
            Status::Yellow => "Y",
            Status::Red => "R",
            Status::Grey => "X",
        }
    }
}

pub enum Error {
    InvalidWordLength,
    InvalidWord,
}

pub struct GameStatus {
    target: [char; WORD_SIZE],
    trials: u32,
    keyboard: [Status; ALPHABET_SIZE],
    count: HashMap<char, u32>,
}

impl GameStatus {
    fn new(target: &str) -> Self {
        let target = target.to_uppercase();
        let mut map = HashMap::new();
        for c in target.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        Self {
            target: target.chars().collect(),
            trials: 0,
            keyboard: [Status::Grey; ALPHABET_SIZE],
            count: map,
        }
    }

    fn guess(&mut self, word: &str) -> Result<bool, Error> {
        let word = word.to_uppercase();
        if word.len() != WORD_SIZE {
            return Err(Error::InvalidWordLength);
        }
        if !ACCEPTABLE.contains(&word.as_str()) {
            return Err(Error::InvalidWord);
        }
        self.trials += 1;
        let word = word.chars();
        let mut status = [Status::Red; WORD_SIZE];
        let mut correct = true;
        // green
        for (i, c) in word.enumerate() {
            if c == self.target[i] {
                status[i] = Status::Green;
                self.keyboard[c as usize - BASE_CHAR as usize] = Status::Green;
            } else {
                correct = false;
            }
        }
        // yellow
        let mut count = self.count.clone();
        for (i, c) in word.enumerate() {
            if status[i] == Status::Red && count[c] > 0 {
                status[i] = Status::Yellow;
                self.keyboard[c as usize - BASE_CHAR as usize] = Status::Yellow;
                count[c] -= 1;
            }
        }
        Ok(correct)
    }
}