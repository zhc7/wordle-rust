use std::cmp::{max, min};
use std::collections::HashMap;

const WORD_SIZE: usize = 5;
const ALPHABET_SIZE: usize = 26;
pub(crate) const BASE_CHAR: char = 'A';


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

#[derive(Debug)]
pub enum Error {
    InvalidWordLength,
    InvalidWord,
}

pub struct GameStatus<'a> {
    pub target: [char; WORD_SIZE],
    pub trials: u32,
    pub keyboard: [Status; ALPHABET_SIZE],
    pub end: bool,
    count: HashMap<char, u32>,
    green_place: [char; WORD_SIZE],
    yellow_count: HashMap<char, u32>,
    acceptable: &'a Vec<String>,
}

impl<'a> GameStatus<'a> {
    pub fn new(target: &str, acceptable: &'a Vec<String>) -> Self {
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
            green_place: [0 as char; WORD_SIZE],
            yellow_count: HashMap::new(),
            acceptable,
        }
    }

    pub fn guess(&mut self, word: &str) -> Result<[Status; WORD_SIZE], Error> {
        if word.len() != WORD_SIZE {
            return Err(Error::InvalidWordLength);
        }
        let word = word.to_uppercase();
        if !self.acceptable.contains(&word) {
            return Err(Error::InvalidWord);
        }
        self.trials += 1;
        let mut status = [Status::Red; WORD_SIZE];
        let mut correct = true;
        let mut count = self.count.clone();
        // green
        for (i, c) in word.chars().enumerate() {
            if c == self.target[i] {
                status[i] = Status::Green;
                self.keyboard[c as usize - BASE_CHAR as usize] = Status::Green;
                count.entry(c).and_modify(|e| *e -= 1);
                self.green_place[i] = c;
            } else {
                correct = false;
            }
        }
        // yellow
        let mut yellow_count = HashMap::new();
        for (i, c) in word.chars().enumerate() {
            if status[i] == Status::Red && count.contains_key(&c) && count[&c] > 0 {
                status[i] = Status::Yellow;
                let key = c as usize - BASE_CHAR as usize;
                self.keyboard[key] = min(self.keyboard[key], Status::Yellow);
                count.entry(c).and_modify(|e| *e -= 1);
                *yellow_count.entry(c).or_insert(0) += 1;
            }
        }
        for (c, count) in yellow_count.iter() {
            self.yellow_count
                .entry(*c)
                .and_modify(|e| *e = max(*e, *count))
                .or_insert(*count);
        }
        // red
        for (i, c) in word.chars().enumerate() {
            if status[i] == Status::Red {
                let key = c as usize - BASE_CHAR as usize;
                self.keyboard[key] = min(self.keyboard[key], Status::Red);
            }
        }
        if correct {
            self.end = true;
        }
        Ok(status)
    }

    pub fn check(&self, word: &str) -> Result<bool, Error> {
        if word.len() != WORD_SIZE {
            return Err(Error::InvalidWordLength);
        }
        let word = word.to_uppercase();
        if !self.acceptable.contains(&word) {
            return Err(Error::InvalidWord);
        }
        for (i, c) in word.chars().enumerate() {
            if self.green_place[i] != 0 as char && c != self.green_place[i] {
                return Ok(false);
            }
        }
        for (c, count) in self.yellow_count.iter() {
            if word.matches(*c).count() < *count as usize {
                return Ok(false);
            }
        }
        Ok(true)
    }
}