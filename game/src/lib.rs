use std::io;
use std::io::Write;
use std::process;
use character::charactersheet::{build_character, Character};
use core::text_handeling::unwrap_str;

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
            println!("Invalid input.");
        } else {
            let mut result: Vec<String> = Vec::new();

            for word in words {
                result.push(word.to_string());
            }

            if parse_input(result) {
                name = input;
                done = true;
            }
        }
    }

    return build_character(name);
}

fn parse_input(words: Vec<String>) -> bool {
    let mut words = words.iter();

    let command = unwrap_str(words.next());

    match command {
        "quit" => {
            println!("Really? Ok, bye!");
            process::exit(1);
        },
        _ => {
            return true;
        }
    }
}
