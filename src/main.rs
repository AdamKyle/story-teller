use std::io;
use std::collections::HashMap;
use std::vec::Vec;
use std::boxed::Box;
use core::launch_adventure::start_adventure;
use core::create_adventures::make_adventure_list;
use core::process_call_backs::SimpleCallback;
use adventures::dark_harvest::launch_dark_harvest;
use game::create_game;
use game::game::set_up_game::Game;

/// Gets a list of adventures and creates them as Hashmap.
fn get_adventures(game: Game) -> HashMap<i32, SimpleCallback> {

    let mut adventures = Vec::new();

    let dark_harvest = SimpleCallback {
        callback: Box::new(|| launch_dark_harvest(game))
    };

    adventures.push(dark_harvest);

    return make_adventure_list(adventures);
}

/// Main loop
fn main() {

    let game = create_game();

    println!("Welcome {}", game.get_character().name);
    println!("====[ Adventures ]====");

    println!("1) Dark Heart");
    println!("\nPlease Choose by typing the number beside the name: ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Invalid input.");

    start_adventure(choice, get_adventures(game));
}
