use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::vec::Vec;
use std::boxed::Box;
use std::process;
use core::launch_adventure::start_adventure;
use core::create_adventures::make_adventure_list;
use core::process_call_backs::SimpleCallback;
use adventures::dark_harvest::run_dark_harvest;
use character::charactersheet::Character;
use game::create_character;

/// Gets a list of adventures and creates them as Hashmap.
///
/// Each of the adeventures are stored in a hash map that stores the number (input)
/// to match against and the callback that launches the adventure when the inout matches.
fn get_adventures(character: Character) -> HashMap<i32, SimpleCallback> {

    let mut adventures = Vec::new();

    let dark_harvest = SimpleCallback {
        callback: Box::new(|| run_dark_harvest(character))
    };

    adventures.push(dark_harvest);

    return make_adventure_list(adventures);
}

fn main() {

    let character = create_character();

    println!("\nWelcome {}, the available adventures for you are:", character.name);
    println!("\n========[ Adventures ]========");
    println!("1) Dark Harvest");
    println!("==============================");
    println!("Please Choose by typing the number beside the name: (You can also type quit to exit) ");

    println!("\nOnce you select an adventure you can then setup your character by selecting a race, class and set your stats.");

    let mut done = false;

    let mut choice = String::new();

    while !done {
        print!("> ");

        io::stdout().flush().expect("Error flushing stdout!");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
                   .expect("Error reading stdin!");

        if input.ends_with('\n') {
           input.pop();
        }

        let words: Vec<&str> = input.split_whitespace().collect();

        if words.is_empty() {
            println!("Invalid input. Try again.");
        } else if input.to_string() == "quit".to_string() {
            println!("Bye now!");
            process::exit(1);
        } else {
            choice = input;
            done = true;
        }
    }

    start_adventure(choice, get_adventures(character));
}
