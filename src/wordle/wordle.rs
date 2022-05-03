use crate::words::dictionary_words;
use colored::Colorize;
use rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;

pub const TOTAL_TRIES: usize = 6;

#[derive(Debug, Default)]
pub struct WordleGame {
    correct_word: String,
    correct_characters: std::collections::HashSet<char>
}

impl WordleGame {
    pub fn new() -> WordleGame {
        let mut rng = rand::thread_rng();
        let correct_word = &dictionary_words()[rng.gen_range(0..dictionary_words().len())];
        let mut correct_characters = HashSet::new();
        for c in correct_word.chars() {
            correct_characters.insert(c);
        }
        return WordleGame {
            correct_word: correct_word.to_string(),
            correct_characters: correct_characters
        };
    }

    pub fn get_correct_word(&self) -> &String {
        return &self.correct_word;
    }

    pub fn get_correct_characters(&self) -> &std::collections::HashSet<char> {
        return &self.correct_characters;
    }

    pub fn print_keyboard(game_character_status: &HashMap<char, i32>) {
        let order = Vec::from([
            'q','w','e','r','t','y','u','i','o','p',
            'a','s','d','f','g','h','j','k','l',
            'z','x','c','v','b','n','m']);
        let mut count = 0;
        print!("   ");
        for i in order {
            if count == 9 {
                print!("\n   ");
            } else if count == 18 {
                print!("\n    ");
            }
    
            if *game_character_status.get(&i).unwrap() == 0 {
                print!("{} ",format!("{}",i).bold());
            } else if *game_character_status.get(&i).unwrap() == 1 {
                print!("{} ",format!("{}",i).yellow().bold());
            } else if *game_character_status.get(&i).unwrap() == 2 {
                print!("{} ",format!("{}",i).green().bold());
            } else if *game_character_status.get(&i).unwrap() == 3 {
                print!("{} ",format!("{}",i).red().bold());
            }
            count += 1;
        }
    }
    
    pub fn print_game_board(game_data: &[Vec<char>], game_data_colored: &[Vec<i32>]) {
    
        println!("-------{}-------\n", format!("Game Board").green().bold());
        let mut i_count = 0;
        for i in game_data {
            print!("       ");
            let mut j_count = 0;
            for j in i {
                if game_data_colored[i_count][j_count] == 2 {
                    print!("{} ",format!("{}",j).green().bold());
                } else if game_data_colored[i_count][j_count] == 1 {
                    print!("{} ",format!("{}",j).yellow().bold());
                } else if game_data_colored[i_count][j_count] == 3 {
                    print!("{} ",format!("{}",j).red().bold());
                } else if game_data_colored[i_count][j_count] == 0 {
                    print!("{} ",j);
                }
                j_count += 1;
            }
            i_count +=1;
            println!();
        }
        println!("\n------------------------");
    }
    
    pub fn verbose_is_valid_input(input: String) -> bool {
        if input.chars().count() != 5 {
            println!("\nYour word must be 5 letters long. Try again!");
            println!("\n========================\n");
            return false;
        }
        for i in input.chars() {
            if !i.is_alphabetic() {
                println!("\nYour word must be composed of alphabetic characters. Try again!");
                println!("\n========================\n");
                return false;
            }
        }
        if !dictionary_words().contains(&input) {
            println!("\nYour word is not on Wordle's word list. Try again!");
            println!("\n========================\n");
            return false;
        }
        return true;
    }
}