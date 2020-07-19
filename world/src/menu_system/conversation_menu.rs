use std::io;
use std::io::Write;
use std::vec::Vec;
use std::collections::HashMap;
use core::text_handeling::unwrap_str;
use crate::world::conversation::{Converse, Choices};
use menu::menu_system::{parse_quit};

pub fn process(options: HashMap<i32, String>, choices: Vec<Choices>) -> Option<Converse> {

    let mut done: bool = false;

    let choice_selection: Option<Converse> = None;

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

            if parse_quit(result.clone()) {
                done = true;
            }

            let choice_selection = parse_choice_input(result.clone(), options.clone(), choices.clone());

            if choice_selection.is_some() {
                done = true;
            }
        }
    }

    return choice_selection;
}

pub fn conversation_menu(choices: Vec<Choices>) -> HashMap<i32, String> {
    let mut options = HashMap::new();

    let mut count: i32 = 1;

    for choice in &choices {
        options.insert(count, choice.clone().choice);
        count = count + 1;
    }

    return options;
}

fn parse_choice_input(words: Vec<String>, options: HashMap<i32, String>, choices: Vec<Choices>) -> Option<Converse> {
    let mut words = words.iter();

    let input = unwrap_str(words.next());

    match input.parse::<i32>() {
        Ok(n) => {
            if options.contains_key(&n) {
                // You can't have a choice that leads to no convo. So if this then
                // explodses thats on you for not supplying a a conversdation to a choice option
                return Some(choices[n as usize - 1].next.clone());
            } else {
                println!("Not a valid choice.");

                return None;
            }
        },
        Err(_e) => {
            match input {
                "quit" | "q" | "exit" => {
                    println!("Conversation over. You can talk again or do other actions in the room. Type help for more information.");
                    return None;
                },
                _ => {
                    println!("invalid input");
                    return None;
                }
            }
        }
    }
}
