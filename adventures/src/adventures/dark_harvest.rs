use std::io;
use std::io::Write;
use game::game::set_up_game::Game;
use world::{Direction,  World, Action, Exit, Room, make_exit};
use text_parsing::parse_input;

/// Launch the dark harvest games.
pub fn launch_dark_harvest(mut game: Game) {
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
    println!("\nCharacter Creation: Help us create your character sheet.");
    print!("{}[2J", 27 as char);

    // Create character sheet here.

    let world = World::new(make_starting_room());

    println!("\n");
    println!("Location: {}", world.get_base_room().name());
    println!("\n");
    println!("{}", world.get_base_room().describe());
    println!("\n");
    println!("What do you do? (type help for commands)");

    while game.is_active() {

        print!("> ");

        io::stdout().flush().expect("Error flushing stdout!");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
                   .expect("Error reading stdin!");

        parse_input(input);

        game.set_active(false);
    }
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
