use std::io;
use std::io::Write;
use std::process;
use character::charactersheet::{build_character, Character};
use core::text_handeling::unwrap_str;
use world::World;
use world::room::{Room, Direction};
use world::actions::Action;
use world::conversation::Converse;

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
    pub stat_bonuses: Vec<i32>,
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
            "look" => self.process_action(Action::Look),
            "explore" => self.process_action(Action::Explore),
            "talk" | "converse" => self.process_action(Action::Talk),
            "q" | "quit" | "exit" => self.quit_game(),
            _ => {
                println!("What is: {}?", command);
                return;
            },
        }
    }

    fn show_help(&mut self) {
        println!("\n-------------------");
        println!("- Movement: Characters can move by typing: go/walk DIRECTION where DIRECTION equals n(orth), s(outh), e(ast) or w(est) or back.");
        println!("- Actions: you can type an action as such: ACTION where action is look, explore or talk.");
        println!("- Quitting: You can quit by typing: q, quit or exit.");
        println!("-------------------");
    }

    fn quit_game(&mut self) {
        println!("\nReally? Ok. Bye.");

        self.active = false;
    }

    fn process_action(&mut self, action: Action) {

        match action {
            Action::Look => self.do_action(Action::Look, self.game_character.stats.clone().unwrap().int),
            Action::Explore => self.do_action(Action::Explore, self.game_character.stats.clone().unwrap().int),
            Action::Talk => self.do_action(Action::Talk, self.game_character.stats.clone().unwrap().chr),
            _ => {
                println!("You have no idea how to do that action.");
                return;
            }
        }
    }

    fn do_action(&mut self, action: Action, stat_value: i32) {
        let room = self.current_room.clone().unwrap();

        if action == Action::Talk && room.conversation.is_some() {
            self.talk(room.conversation);
        } else {
            let bonus = self.stat_bonuses[stat_value as usize];

            room.do_action(action, bonus);
        }
    }

    fn talk(&mut self, conversation: Option<Converse>) {

        if conversation.clone().is_some() {
            println!("\n{}", conversation.clone().unwrap().line);

            let choices = conversation.clone().unwrap().choices;

            if choices.is_some() {
                conversation.clone().unwrap().process_conversation();
            }

        } else {
            println!("Sure, talk to your self. That's not crazy at all. I do it. All the time.");
        }
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


            if !room.go_back.can_go_back {
                if room.go_back.reason.is_some() {
                    println!("{}", room.go_back.reason.clone().unwrap());
                    return;
                } else {
                    println!("You cannot go back. What now?");
                    return;
                }
            }

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
        println!("\nWhat do you do? (type help for commands)");
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
