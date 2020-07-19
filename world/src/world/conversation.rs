use std::io;
use std::io::Write;
use std::vec::Vec;
use std::collections::HashMap;
use core::text_handeling::unwrap_str;

/// Converseations consisting of a line and possible choices.
///
/// The line is what the NPC sais. The choices are a Option of vector choices.
/// each choice also has a choice, this is for the player to see and a optional conversation (Converse)
/// struct called next.
///
/// This is considered, much like the way rooms are built, to be a top down recursive struct
/// that is traversed until the final Converse with no choice is reached.
///
/// TODO: Choices should make next mandatory, since you cant have a choice without a response.
#[derive(Clone, Debug)]
pub struct Converse {
    pub line: String,
    pub choices: Option<Vec<Choices>>,
}

impl Converse {

    /// Create a new instance of Converse
    pub fn new(line: String, choices: Option<Vec<Choices>>) -> Self {
        Converse {
            line: line,
            choices: choices,
        }
    }

    /// Process the conversation.
    ///
    /// Assuming a conversation has some choices we will loop ove the choices,
    /// allowing the user to leave the conversation at any point by typing q, quit, exit.
    ///
    /// This method is recurisve until the final conversation struct is found with no choices.
    /// Then we return to the main loop.
    pub fn process_conversation(&mut self) {

        if !self.choices.clone().is_some() {
            return;
        }

        let choices = self.choices.clone().unwrap();

        let mut options = HashMap::new();

        let mut count: i32 = 1;

        println!("\n===== [Choices] =====");

        for choice in &choices {
            options.insert(count, choice.clone().choice);

            print!("{}) {}", count, choice.clone().choice);

            count = count + 1;
        }

        println!("\n=====================");
        println!("Choose one by typing the number or q, quit or exit to leave the conversation.");
        println!("\n");

        let mut done: bool = false;

        let mut next: Option<Converse> = None;

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

                if self.parse_quit(result.clone()) {
                    done = true;
                }

                let choice_selection = self.parse_choice_input(result.clone(), options.clone(), choices.clone());

                if choice_selection.is_some() {
                    next = choice_selection;
                    done = true;
                }
            }
        }

        if (next.is_some()) {
            println!("{}", next.clone().unwrap().line);
            next.clone().unwrap().process_conversation();
        }
    }

    fn parse_choice_input(&mut self, words: Vec<String>, options: HashMap<i32, String>, choices: Vec<Choices>) -> Option<Converse> {
        let mut words = words.iter();

        let input = unwrap_str(words.next());

        match input.parse::<i32>() {
            Ok(n) => {
                if options.contains_key(&n) {
                    // You can't have a choice that leads to no convo. So if this then
                    // explodses thats on you for not supplying a a conversdation to a choice option
                    return Some(choices[n as usize - 1].next.clone().unwrap());
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

    fn parse_quit(&mut self, words: Vec<String>) -> bool {
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
}

/// Choices for the conversation.
///
/// Conversations can usually have choices which are stored in a vector.
/// each choice has a string which is what the player sees in a list of choices.
///
/// The next aspect of the struct contiues the conversation.
#[derive(Clone, Debug)]
pub struct Choices {
    pub choice: String,
    pub next: Option<Converse>,
}

impl Choices {

    /// Create a new choice
    pub fn new(choice: String, next: Option<Converse>) -> Self {
        Choices {
            choice: choice,
            next: next,
        }
    }
}
