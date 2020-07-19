use std::collections::HashMap;
use crate::core::process_call_backs::SimpleCallback;
use crate::text_handeling::unwrap_str;

pub fn menu(base: SimpleCallback) {
    base.process();
}

pub fn display_menu(choices: &mut HashMap<i32, String>) {

    println!("\n===== [Choices] =====");

    for (key, value) in &*choices {
        print!("{}) {}", key, value);
    }

    println!("\n=====================");
    println!("Choose one by typing the number or q, quit or exit to leave the conversation.");
    println!("\n");
}

pub fn parse_quit(words: Vec<String>) -> bool {
    let mut words = words.iter();

    let command = unwrap_str(words.next());

    match command {
        "quit" | "q" | "exit" => {
            println!("You abruptly left the conversation. You can talk again or do other actions in the room. Type help for more information.");
            return true;
        },
        _ => {
            return false;
        }
    }
}
