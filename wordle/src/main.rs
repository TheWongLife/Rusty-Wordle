//Declare dependencies
use std::io::stdin;

fn main() {
    println!("\nWelcome to Rusty Wordle!\nInstructions:\n");
    println!("\n========================\n");

    let mut tries = 6;
    while tries > 0 {
        println!("{} tries left. Enter a valid 5 letter word: ", tries);
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string);
        input_string = input_string[0..input_string.len()-1].to_string();
        if verbose_is_valid_input(input_string) {
            tries -= 1;
        }
    }
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
    return true;
}