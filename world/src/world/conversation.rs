use std::io;
use std::io::Write;
use std::vec::Vec;
use std::boxed::Box;
use std::collections::HashMap;
use core::text_handeling::unwrap_str;
use core::process_call_backs::SimpleCallback;
use menu::menu_system::{display_menu, parse_quit};
use crate::menu_system::conversation_menu::{conversation_menu, process};

/// Converseations consisting of a line and possible choices.
///
/// The line is what the NPC sais. The choices are a Option of vector choices.
/// each choice also has a choice, this is for the player to see and a optional conversation (Converse)
/// struct called next.
///
/// This is considered, much like the way rooms are built, to be a top down recursive struct
/// that is traversed until the final Converse with no choice is reached.
///
/// A typical conversation structure looks as such:
///
///             Converse
///                | - Line that starts conversation
///   Vec<Choices> - choice, next (optional converse) - TODO: Make non optional
///                |
///    -------------------------
///     |          |        |
///  converse   converse  converse
///
/// When process_conversation is called, we recurisvely call the process_conversation
/// until a Converse doesnt have a choice. All Choices on a Converse object are Optional.
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

        let mut options = conversation_menu(choices.clone());

        display_menu(&mut options);

        let next = process(options, choices.clone());

        // The user might have exited from a sub menu,
        // such as a choice. This would cancel the whole: talk
        // command making next now be None and causing a panic.
        if next.is_some() {
            println!("{}", next.clone().unwrap().line);
            next.clone().unwrap().process_conversation();
        }
    }
}

/// Choices for the conversation.
///
/// Conversations can usually have choices which are stored in a vector.
/// each choice has a string which is what the player sees in a list of choices.
///
/// The next aspect of the struct contiues the conversation and should be a response to
/// the selected choice.
#[derive(Clone, Debug)]
pub struct Choices {
    pub choice: String,
    pub next: Converse,
}

impl Choices {

    /// Create a new choice
    pub fn new(choice: String, next: Converse) -> Self {
        Choices {
            choice: choice,
            next: next,
        }
    }
}
