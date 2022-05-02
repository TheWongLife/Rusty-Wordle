mod words;

use colored::Colorize;
use std::io::stdin;
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let crab_emoji = '\u{1F980}';
    println!("\nWelcome to {} {} {}!\nInstructions:\n", 
        format!("Rusty").truecolor(255, 165, 0).bold(),
        crab_emoji,
        format!("Wordle").green().bold());
    println!("\n========================\n");

    let mut tries = 6;
    let mut rng = rand::thread_rng();
    let correct_word = &words::dictionary_words()[rng.gen_range(0..words::dictionary_words().len())];
    let mut correct_characters = HashSet::new();
    for i in correct_word.chars() {
        correct_characters.insert(i);
    }
    println!("{}", correct_word);

    //2d vector contains game data
    let mut game_data = vec![vec!['*'; 5];6];
    //set contains user's guessed characters
    while tries > 0 {
        println!("{} tries left. Enter a valid 5 letter word: ", tries);
        let mut input_string = String::new();
        stdin().read_line(&mut input_string);
        input_string = input_string[0..input_string.len()-1].to_string();
        if verbose_is_valid_input(input_string.clone()) {
            for i in 0..5 {
                game_data[6-tries][i] = input_string.clone().chars().nth(i).unwrap();
            }

            print_game_board(&game_data);
            //print_keyboard();
            println!("\n=======================\n");
            tries -= 1;
        }
    }
}

fn print_game_board(game_data: &[Vec<char>]) {

    println!("-------{}-------\n", format!("Game Board").green().bold());
    for i in game_data {
        print!("         ");
        for j in i {
            print!("{}",j);
        }
        println!();
    }
    println!("\n-----------------------");
}

fn verbose_is_valid_input(input: String) -> bool {
    if input.chars().count() != 5 {
        println!("\nYour word must be 5 letters long. Try again!");
        println!("\n=======================n");
        return false;
    }
    for i in input.chars() {
        if !i.is_alphabetic() {
            println!("\nYour word must be composed of alphabetic characters. Try again!");
            println!("\n=======================n");
            return false;
        }
    }
    if !words::dictionary_words().contains(&input) {
        println!("\nYour word is not on Wordle's word list. Try again!");
        println!("\n=======================n");
        return false;
    }
    return true;
}