use std::io;
use std::io::Write;
use std::process;
use character::charactersheet::{build_character, Character};
use core::text_handeling::unwrap_str;
use world::{World, Room};

/// Core Game Struct
///
/// Responsible for holding the character and status of the game.
///
/// This is the most integral part of the game as its the entry to the main game loop.
/// Each adventure would have its own World struct passed into the game that is then acted upon to
/// move the player through the world passed in.
#[derive(Debug)]
pub struct Game {
    pub active: bool,
    pub game_character: Character,
    pub current_room: Option<Room>,
    pub previous_room: Option<Room>
}

/// Game implementation.
impl Game {

    /// Is game active?
    pub fn is_active(&self) -> bool {
        return self.active;
    }

    /// Set the game to either active or not.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Run the loop.
    pub fn run(&mut self, world: World) {

        self.set_base_room(&world);

        while self.active {
            print!("> ");

            io::stdout().flush().expect("Error flushing stdout!");

            let mut input = String::new();

            io::stdin().read_line(&mut input)
                       .expect("Error reading stdin!");

            self.parse_input(input);
        }
    }

    fn set_base_room(&mut self, world: &World) {

        let room = world.room.clone();

        self.current_room = Some(room);
    }

    fn parse_input(&mut self, input: String) {
        let words: Vec<&str> = input.split_whitespace().collect();

        if words.is_empty() {
            println!("Invalid input.");
        }

        let mut result: Vec<String> = Vec::new();

        for word in words {
            let single_word = self.clean_words(word.to_string());

            result.push(single_word);
        }

        self.parse_commands(result);
    }

    fn clean_words(&mut self, word: String) -> String {
        let mut s = String::new();

        for character in word.chars() {
            for d in character.to_lowercase() {
                s.push(d);
            }
        }

        return s;
    }

    fn parse_commands(&mut self, words: Vec<String>) {
        let mut words = words.iter();

        let command = unwrap_str(words.next());

        if command == "" {
            println!("Invalid input.");

            return;
        }

        //let command_one = unwrap_str(words.next());
        //let command_two = unwrap_str(words.next());

        match command {
            "help" => self.show_help(),
            "q" | "quit" | "exit" => self.quit_game(),
            _ => {
                println!("What is: {}?", command);
            },
        }
    }

    fn show_help(&mut self) {
        println!("\n-------------------");
        println!("\nMenu: Help");
        println!("\nMovement: Characters can move by typing: go DIRECTION where DIRECTION equals north, south, east or west.");
        println!("\nActions: you can type an action: ACTION where action is look or explore.");
        println!("\nQuitting: You can quit by typing: q, quit or exit.");
        println!("\n-------------------");
    }

    fn quit_game(&mut self) {
        println!("\nReally? Ok. Bye.");

        self.active = false;
    }
}

/// Creates a character.
///
/// This will return a partial character struct with just the name.
///
/// Will run a loop until the player either enters there name.
///
/// The given options for the input are: name or quit.
pub fn create_character() -> Character {
    println!("Whats your name? (you can type quit, q or exit to exit)");

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
        "quit" | "q" | "exit" => {
            println!("Really? Ok, bye!");
            process::exit(1);
        },
        _ => {
            return true;
        }
    }
}
