use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::collections::HashSet;

/*
Tile enum to represent colors (Can also use a crate for this)
Pick random word from a word bank to start the list using rand choose
Board struct contains random word chosen, map of the position of every letter,
set of all words guessed by player, and number of guesses left
*/

#[derive(Debug)]
pub enum Tile {
    Green,
    Yellow,
    Gray
}

#[derive(Debug, Default)]
pub struct Board {
    word: String,
    letter_positions: std::collections::HashMap<u32, char>,
    guessed_words: std::collections::HashSet<String>.
    guesses_left: usize,
}

impl Board {
    //New board
    pub fn new(word: String) -> Result<Board> {
        if word.is_empty() || word.chars().count() != 5 {
            return Err();
        }
        let mut letter_positions: HashMap<u32, char> = HashMap::new();
        let mut position: u32 = 0;
        for c in word.chars() {
            letter_positions.insert(position, c);
            position += 1;
        }
        return Ok(Board {
            word: word.to_lowercase(),
            letter_positions: letter_positions,
            guessed_words: HashSet::new();
            guesses_left: 6
        });
    }

    //Guess
    pub fn guess(&mut self, word: String) -> Result<bool> {
        if word.chars().count() != 5 {
            return Err();
        }
        if self.guessed_words.contains(word.to_lowercase()) {
            return Err();
        }
        self.guessed_words.insert(word.to_lowercase());
        if word.to_lowercase() == self.word {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    //Game result
    pub fn game_result(&self) -> Option<bool> {
        if self.guesses_left == 0 {
            return Some(false);
        }
        if self.guesses_left != 0 && self.guessed_words.contains(self.word) {
            return Some(true);
        }
        return None();
    }

    //Getters
    pub fn get_word(&self) -> &String {
        return &self.word;
    }

    pub fn get_letter_positions(&self) -> &std::collections::HashMap<u32, char> {
        return &self.letter_positions;
    }

    pub fn get_guessed_words(&self) -> &std::collections::HashSet<String> {
        return &self.guessed_words;
    }

    pub fn get_guesses_left(&self) -> &usize {
        return &self.guesses_left;
    }
}