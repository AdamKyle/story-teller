use std::io;
use character::charactersheet::{build_character, Character};

pub fn create_game() -> Character {
    println!("Whats your name?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Error! Invalid input.");

    let character_name: Vec<&str> = name.split_whitespace().collect();

    if character_name.is_empty() {
        println!("Invalid input.");
    }

    if name.ends_with('\n') {
        name.pop();
    }

    return build_character(name);
}
