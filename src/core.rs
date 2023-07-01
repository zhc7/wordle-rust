use std::cmp::max;
use std::collections::HashMap;
use crate::builtin_words::ACCEPTABLE;

const WORD_SIZE: usize = 5;
const ALPHABET_SIZE: usize = 26;
const BASE_CHAR: char = 'A';


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Green,
    Yellow,
    Red,
    Grey,
}

impl Status {
    pub fn to_str(&self) -> &'static str {
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
    pub target: [char; WORD_SIZE],
    pub trials: u32,
    pub keyboard: [Status; ALPHABET_SIZE],
    pub end: bool,
    count: HashMap<char, u32>,
}

impl GameStatus {
    pub fn new(target: &str) -> Self {
        let target = target.to_uppercase();
        let mut map = HashMap::new();
        for c in target.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        Self {
            target: target.chars().collect::<Vec<char>>().try_into().unwrap(),
            trials: 0,
            keyboard: [Status::Grey; ALPHABET_SIZE],
            end: false,
            count: map,
        }
    }

    pub fn guess(&mut self, word: &str) -> Result<[Status; WORD_SIZE], Error> {
        let word = word.to_uppercase();
        if word.len() != WORD_SIZE {
            return Err(Error::InvalidWordLength);
        }
        if !ACCEPTABLE.contains(&word.as_str()) {
            return Err(Error::InvalidWord);
        }
        self.trials += 1;
        let mut status = [Status::Red; WORD_SIZE];
        let mut correct = true;
        // green
        for (i, c) in word.chars().enumerate() {
            if c == self.target[i] {
                status[i] = Status::Green;
                self.keyboard[c as usize - BASE_CHAR as usize] = Status::Green;
            } else {
                correct = false;
            }
        }
        // yellow
        let mut count = self.count.clone();
        for (i, c) in word.chars().enumerate() {
            if status[i] == Status::Red && count[&c] > 0 {
                status[i] = Status::Yellow;
                let key = c as usize - BASE_CHAR as usize;
                self.keyboard[key] = max(self.keyboard[key], Status::Yellow);
                count.entry(c).and_modify(|e| *e -= 1);
            }
        }
        // red
        for (i, c) in word.chars().enumerate() {
            if status[i] == Status::Red {
                let key = c as usize - BASE_CHAR as usize;
                self.keyboard[key] = max(self.keyboard[key], Status::Red);
            }
        }
        if correct {
            self.end = true;
        }
        Ok(status)
    }
}