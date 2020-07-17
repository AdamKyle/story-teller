use std::io;
use std::io::Write;
use std::process;
use character::charactersheet::{build_character, Character};
use core::text_handeling::unwrap_str;
use world::{World, Room, Direction};

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

    fn set_current_room(&mut self, room: Room) {
        self.current_room = Some(room);
    }

    fn set_previous_room(&mut self, room: Room) {
        self.previous_room = Some(room);
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

        let command_one = unwrap_str(words.next());
        let command_two = unwrap_str(words.next());

        match command {
            "help" => self.show_help(),
            "go" | "walk" | "move" => self.leave_room(command_one),
            "q" | "quit" | "exit" => self.quit_game(),
            _ => {
                println!("What is: {}?", command);
            },
        }
    }

    fn show_help(&mut self) {
        println!("\n-------------------");
        println!("\nMenu: Help");
        println!("\nMovement: Characters can move by typing: go/walk DIRECTION where DIRECTION equals n(orth), s(outh), e(ast) or w(est) or back.");
        println!("\nActions: you can type an action: ACTION where action is look or explore.");
        println!("\nQuitting: You can quit by typing: q, quit or exit.");
        println!("\n-------------------");
    }

    fn quit_game(&mut self) {
        println!("\nReally? Ok. Bye.");

        self.active = false;
    }

    fn leave_room(&mut self, command: &str) {

        let direction_to_go: Direction;

        match command {
            "n" | "north" => direction_to_go = Direction::N,
            "s" | "south" => direction_to_go = Direction::S,
            "e" | "east" => direction_to_go = Direction::E,
            "w" | "west" => direction_to_go = Direction::W,
            "back" => direction_to_go = Direction::BACK,
            _ => {
                println!("You cannot go that way. Please try again.");
                return;
            }
        }

        // if we dont have a current room in this game,
        // then we have broken some where and this should
        // be allowed to panic.
        let room = self.current_room.clone().unwrap();

        if direction_to_go == Direction::BACK {
            // Lets handel going backwards
            //
            // TODO: We need a way to store previvious rooms
            // As you move forward, then pop them off.
            // previous rooms should be a vec of rooms that we pop
            // and push rooms we have previously been in.
            if !self.previous_room.is_some() {
                println!("You turn around to head back, only to discover there is no way back. What now?");
            } else {
                let previous_room = self.current_room.clone().unwrap();
                let current_room = self.previous_room.clone().unwrap();

                self.set_previous_room(previous_room);
                self.set_current_room(current_room);

                let current_room = self.current_room.clone().unwrap();

                self.enter_new_room(current_room);
            }
        } else {
            let new_room = room.exit(direction_to_go);

            if new_room.is_some() {
                self.set_previous_room(room);
                self.set_current_room(new_room.clone().unwrap());

                let current_room = self.current_room.clone().unwrap();

                self.enter_new_room(current_room);
            } else {
                println!("You can't go that way.");
            }
        }
    }

    fn enter_new_room(&mut self, current_room: Room) {
        println!("\n");
        println!("Location: {}", current_room.name());
        println!("\n");
        println!("{}", current_room.describe());
        println!("What do you do? (type help for commands)");
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
