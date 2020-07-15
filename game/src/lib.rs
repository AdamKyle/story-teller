pub mod game;

use std::io;
use character::charactersheet::build_character;

pub use crate::game::set_up_game::Game;

pub fn create_game() -> Game {
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

    let character = build_character(name);

    Game  {
        active: true,
        game_character: character,
    }
}
