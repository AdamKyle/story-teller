use std::io;
use std::collections::HashMap;
use std::vec::Vec;
use character::charactersheet::build_character;
use core::launch_adventure::start_adventure;
use core::create_adventures::make_adventure_list;
use core::process_call_backs::SimpleCallback;
use adventures::dark_harvest::launch_dark_harvest;
use game::create_game;

/// Gets a list of adventures and creates them as Hashmap.
fn get_adventures() -> HashMap<i32, SimpleCallback> {

    let mut adventures = Vec::new();

    let dark_harvest = SimpleCallback {
        callback: launch_dark_harvest
    };

    adventures.push(dark_harvest);

    return make_adventure_list(adventures);
}

/// Main loop
fn main() {
    println!("Whats your name?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("What?");

    let character = build_character(name);

    let game = create_game(character);

    println!("Welcome {}", game.get_character().name);
    println!("====[ Adventures ]====");

    println!("1) Dark Heart");
    println!("\nPlease Choose by typing the number beside the name: ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("What?");

    start_adventure(choice, get_adventures());
}
