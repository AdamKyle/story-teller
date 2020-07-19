use std::io;
use std::io::Write;
use std::vec::Vec;
use std::collections::HashMap;
use core::text_handeling::unwrap_str;
use world::person::{Person};


pub fn menu_choices(people: Vec<Person>) -> HashMap<i32, String> {
    let mut options = HashMap::new();

    let mut count: i32 = 1;

    for people in &people {
        options.insert(count, people.clone().name);
        count = count + 1;
    }

    return options;
}

pub fn process(options: HashMap<i32, Person>) -> Option<Person> {

    let mut done: bool = false;

    let mut found_person: Option<Person> = None;

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

            let person = parse_choice_input(result.clone(), options.clone());

            if person.is_some() {
                found_person = person;
                done = true;
            }
        }
    }

    return found_person;
}

fn parse_choice_input(words: Vec<String>, options: HashMap<i32, Person>) -> Option<Person> {
    let mut words = words.iter();

    let input = unwrap_str(words.next());

    match input.parse::<i32>() {
        Ok(n) => {
            if options.contains_key(&n) {
                return Some(options[&n].clone());
            } else {
                println!("Not a valid choice.");

                return None;
            }

        },
        Err(_e) => {
            match input {
                "quit" | "q" | "exit" => {
                    println!("Conversation over.");
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

fn parse_quit(words: Vec<String>) -> bool {
    let mut words = words.iter();

    let command = unwrap_str(words.next());

    match command {
        "quit" | "q" | "exit" => {
            println!("You turn away from the people. You can talk again or do other actions in the room. Type help for more information.");
            return true;
        },
        _ => {
            return false;
        }
    }
}
