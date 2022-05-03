pub mod words;
pub mod wordle;

use crate::wordle::wordle::{WordleGame, TOTAL_TRIES};
use colored::Colorize;
use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let crab_emoji = '\u{1F980}';
    let wong = '\u{1F440}';
    let thumbs_up = '\u{1F44D}';
    println!("\nWelcome to {} {} {}!\nInstructions:
    \nGuess a Five Letter Word!
    \nAfter Every Guess, Each Letter Is Marked a Color
    \nThe letter is {} if it is in the answer and in the correct position
    \nThe letter is {} if it is in the answer but not in the right position
    \nThe letter is {} if it is not in the answer at all
    \nIf you Fail to Guess the Word in Six Tries, You {} {}\n{} {}", 
        format!("Rusty").truecolor(255, 165, 0).bold(),
        crab_emoji,
        format!("Wordle").green().bold(),
        format!("Green").green().bold(),
        format!("Yellow").yellow().bold(),
        format!("Gray").truecolor(25, 25, 25).bold(),
        //If anyone knows how to make this gray pls fix
        format!("Lose").red().bold(),
        wong,
        format!("Good Luck!").green().bold(),
        thumbs_up

    );
    println!("\n========================\n");

    let wordle_game = WordleGame::new();
    println!("[FOR DEMO PURPOSES] Answer: {}", wordle_game.get_correct_word()); //comment to hide answer

    //2d vector contains game data
    let mut game_data = vec![vec!['*'; 5];6];
    let mut game_data_colored = vec![vec![0; 5];6];
    let mut game_character_status = HashMap::from([
        ('q',0),('w',0),('e',0),('r',0),('t',0),('y',0),('u',0),('i',0),('o',0),('p',0),
        ('a',0),('s',0),('d',0),('f',0),('g',0),('h',0),('j',0),('k',0),('l',0),
        ('z',0),('x',0),('c',0),('v',0),('b',0),('n',0),('m',0)
    ]);
    let mut tries = TOTAL_TRIES;
    while tries > 0 {
        println!("{} tries left. Enter a valid 5 letter word: ", tries);
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).expect("Couldn't read input from stdin");
        input_string = input_string[0..input_string.len()-1].to_string();
        if WordleGame::verbose_is_valid_input(input_string.clone()) {
            for i in 0..5 {
                game_data[6-tries][i] = input_string.clone().chars().nth(i).unwrap();
                if input_string.clone().to_lowercase().chars().nth(i).unwrap() == wordle_game.get_correct_word().clone().chars().nth(i).unwrap() {
                    game_data_colored[6-tries][i] = 2; //exact match (character & location)
                    game_character_status.insert(input_string.clone().to_lowercase().chars().nth(i).unwrap(),2);
                } else if wordle_game.get_correct_characters().contains(&input_string.clone().to_lowercase().chars().nth(i).unwrap()) {
                    game_data_colored[6-tries][i] = 1; //location mismatch
                    game_character_status.insert(input_string.clone().to_lowercase().chars().nth(i).unwrap(),1);
                } else {
                    game_data_colored[6-tries][i] = 3; //character mismatch
                    game_character_status.insert(input_string.clone().to_lowercase().chars().nth(i).unwrap(),3);
                }
            }

            WordleGame::print_game_board(&game_data, &game_data_colored);
            WordleGame::print_keyboard(&game_character_status);
            println!("\n========================\n");
            if input_string.clone() == wordle_game.get_correct_word().clone() {
                println!("\n========================");
                if 7 - tries == 1 {
                    println!("{}, you found the word in 1 try \u{1F914}",
                    format!("Wow").green().bold());
                } else {
                    println!("{}, you found the word in {} tries \u{1F389}",
                        format!("Congratulations").green().bold(),
                        7 - tries);
                }
                println!("========================\n");
                return;
            }
            tries -= 1;
        }
    }

    println!("\n========================");
    println!("The word was {}! You failed this time, but you can try again!", wordle_game.get_correct_word());
    println!("========================\n");
    return;
}
