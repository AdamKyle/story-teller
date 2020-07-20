use std::collections::HashMap;
use core::text_handeling::unwrap_str;

/// Display the menu for the player to make choices from.
pub fn display_menu(choices: &mut HashMap<i32, String>) {

    println!("\n===== [Choices] =====");

    for (key, value) in &*choices {
        print!("{}) {}", key, value);
    }

    println!("\n=====================");
    println!("Choose one by typing the number or q, quit or exit to leave the conversation.");
    println!("\n");
}

/// Parsing the quit options
///
/// When the user types quit, q or exit the menu system will
/// quit.
///
/// Only call this method with in the context of the choice
/// selection loop. the idea is to kill the loop.
pub fn parse_quit(words: Vec<String>) -> bool {
    let mut words = words.iter();

    let command = unwrap_str(words.next());

    match command {
        "quit" | "q" | "exit" => {
            println!("You abruptly left the conversation.");
            return true;
        },
        _ => {
            return false;
        }
    }
}
