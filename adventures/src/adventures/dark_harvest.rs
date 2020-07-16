use std::io;
use std::io::Write;
use world::{Direction,  World, Action, Exit, Room, make_exit};
use character::charactersheet::{Character, create_stats};
use core::text_handeling::unwrap_str;


#[derive(Debug)]
pub struct Game {
    pub active: bool,
    pub game_character: Character,
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

    pub fn run(&mut self) {
        while self.active {
            print!("> ");

            io::stdout().flush().expect("Error flushing stdout!");

            let mut input = String::new();

            io::stdin().read_line(&mut input)
                       .expect("Error reading stdin!");

            self.parse_input(input);
        }
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

pub fn run_dark_harvest(mut character: Character) {

    println!("\nCharacter Creation: Help us create your character sheet.");

    character = create_stats(character);

    let mut _world = dark_harvest_intro();

    let mut game = Game {
        active: true,
        game_character: character,
    };

    game.run();
}

fn dark_harvest_intro() -> World {
    print!("{}[2J", 27 as char);
    println!("==== [Dark Harvest] ===");
    println!("\nIntroduction:");
    println!("
    Welcome to Dark Harvest. This is the first chapter in the story of: The Child and The Poet, a dark story
    revolving around love, loss and other worlds. While inspired by DND, Dark Harvest doesnt follow any rule books
    closely. Only as a reference.
    ");
    println!("Story:");
    println!("
    You awaken in the middle of a field. Covered in blood, you have no idea how you got here. As you stand and adjust your eyes to the
    blinding sun, you feel your body for wounds, to find the source of the bleeding. No wounds present them selves.\n
    \n
    The wind kicks up and a voice is heard, like a whisper moving through the shadows, Who are you?
    ");
    println!("\n\nAdventure Difficulty: Easy");
    println!("Adventure Lenth:      Medium");
    println!("=======================");

    let world = World::new(make_starting_room());

    println!("\n");
    println!("Location: {}", world.get_base_room().name());
    println!("\n");
    println!("{}", world.get_base_room().describe());
    println!("\n");
    println!("What do you do? (type help for commands)");

    return world;
}


fn make_starting_room() -> Room {
    let mut actions = Vec::new();

    actions.push(Action::Explore);

    let mut exits = Vec::new();

    exits.push(make_exit(Direction::N, make_path_way()));

    return Room::new(
        "Grassy Clearing".to_string(),
        "The sound of birds, the rustling of the wind. The warmth of the sun in the blue sky. The grassy clearing is clear, trees to the east, a small path to the north. South an west contain ruins and broken peices of what appears to be concrete".to_string(),
        actions,
        exits,
    );
}

fn make_path_way() -> Room {
    let mut actions = Vec::new();

    actions.push(Action::NONE);

    let mut exits = Vec::new();

    exits.push(Exit {
        direction: Direction::NONE,
        room: None
    });

    return Room::new(
        "Path".to_string(),
        "As you walk up the path the trees around seem to get thicker, taller and the area darker with shade.".to_string(),
        actions,
        exits,
    );
}
