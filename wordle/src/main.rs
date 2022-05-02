mod words;

use colored::Colorize;
use std::io::stdin;
use rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let crab_emoji = '\u{1F980}';
    println!("\nWelcome to {} {} {}!\nInstructions:\n[TODO]", 
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
    println!("Answer: {}", correct_word); //uncomment to find answer

    //2d vector contains game data
    let mut game_data = vec![vec!['*'; 5];6];
    let mut game_data_colored = vec![vec![0; 5];6];
    let mut game_character_status = HashMap::from([
        ('q',0),('w',0),('e',0),('r',0),('t',0),('y',0),('u',0),('i',0),('o',0),('p',0),
        ('a',0),('s',0),('d',0),('f',0),('g',0),('h',0),('j',0),('k',0),('l',0),
        ('z',0),('x',0),('c',0),('v',0),('b',0),('n',0),('m',0)
    ]);
    //set contains user's guessed characters
    while tries > 0 {
        println!("{} tries left. Enter a valid 5 letter word: ", tries);
        let mut input_string = String::new();
        stdin().read_line(&mut input_string);
        input_string = input_string[0..input_string.len()-1].to_string();
        if verbose_is_valid_input(input_string.clone()) {
            if input_string.clone() == correct_word.clone() {
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
            for i in 0..5 {
                game_data[6-tries][i] = input_string.clone().chars().nth(i).unwrap();
                if input_string.clone().to_lowercase().chars().nth(i).unwrap() == correct_word.clone().chars().nth(i).unwrap() {
                    game_data_colored[6-tries][i] = 2; //exact match (character & location)
                    game_character_status.insert(input_string.clone().to_lowercase().chars().nth(i).unwrap(),2);
                }
                else if correct_characters.contains(&input_string.clone().to_lowercase().chars().nth(i).unwrap()) {
                    game_data_colored[6-tries][i] = 1; //location mismatch
                    game_character_status.insert(input_string.clone().to_lowercase().chars().nth(i).unwrap(),1);
                }
            }

            print_game_board(&game_data, &game_data_colored);
            print_keyboard(&game_character_status);
            println!("\n========================\n");
            tries -= 1;
        }
    }
    println!("\n========================");
    println!("The word was {}! You failed this time, but you can try again!", correct_word);
    println!("========================\n");
    return;
}

fn print_keyboard(game_character_status: &HashMap<char, i32>) {
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
        }

        count += 1;
    }
    //10
    //9
    //7
    /*println!( {} {} {} {} {} {} {} {} {} {});
    println!(  {} {} {} {} {} {} {} {} {});
    println!(   {} {} {} {} {} {} {} {});*/
}

fn print_game_board(game_data: &[Vec<char>], game_data_colored: &[Vec<i32>]) {

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

fn verbose_is_valid_input(input: String) -> bool {
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
    if !words::dictionary_words().contains(&input) {
        println!("\nYour word is not on Wordle's word list. Try again!");
        println!("\n========================\n");
        return false;
    }
    return true;
}