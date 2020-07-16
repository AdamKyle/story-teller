use std::io;
use std::io::Write;
use std::process;
use character::charactersheet::{build_character, Character};

/// Creates a character.
///
/// This will return a partial character struct with just the name.
///
/// Will run a loop until the player either enters there name.
///
/// The given options for the input are: name or quit.
pub fn create_character() -> Character {
    println!("Whats your name? (you can type quit to exit)");

    let mut done = false;

    let mut name: String = String::new();

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
            name = input;
            done = true;
        }
    }

    return build_character(name);
}
