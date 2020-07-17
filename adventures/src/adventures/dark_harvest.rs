use std::collections::HashMap;
use world::{Direction,  World, Action, Exit, Room, OnAction, GoBack, make_exit};
use character::charactersheet::{Character, create_stats};
use game::Game;
use core::stat_bonus::create_all_stat_bonuses;

/// This is the actual game adventure It's self.
///
/// This adventure will start with you creating your character before describing the Dark Harvest
/// world. Finally we create the game object and run the game.
pub fn run_dark_harvest(mut character: Character) {

    println!("\nCharacter Creation: Help us create your character sheet.");

    character = create_stats(character);

    let world = dark_harvest_intro();

    let mut game = Game {
        active: true,
        game_character: character,
        stat_bonuses: create_all_stat_bonuses(),
        current_room: None,
        previous_room: None,
    };

    game.run(world);
}

fn dark_harvest_intro() -> World {
    println!("==== [Dark Harvest] ===");
    println!("\nIntroduction:");
    println!("
    Welcome to Dark Harvest. This is the first chapter in the story of: The Child and The Poet, a dark story
    revolving around love, loss and other worlds. While inspired by DND, Dark Harvest doesn't follow any rule books
    closely. Only as a reference.
    ");
    println!("Story:");
    println!("\nYou awaken in the middle of a field. Covered in blood, you have no idea how you got here. As you stand and adjust your eyes to the blinding sun, you feel your body for wounds, to find the source of the bleeding. No wounds present them selves.");
    println!(r#"The wind kicks up and a voice is heard, like a whisper moving through the shadows, "Who are you?""#);
    println!("\n\nAdventure Difficulty: Easy");
    println!("Adventure Length:      Medium");
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
    let mut actions = HashMap::new();

    actions.insert(Action::Explore, Some(OnAction::new("You look around and see nothing of interest.".to_string(), None)));

    let mut exits = Vec::new();

    exits.push(make_exit(Direction::N, make_path_way()));

    return Room::new(
        "Grassy Clearing".to_string(),
        "The sound of birds, the rustling of the wind. The warmth of the sun in the blue sky. The grassy clearing is clear, trees to the east, a small path to the north. South an west contain ruins and broken peices of what appears to be concrete".to_string(),
        actions,
        exits,
        GoBack::new(true, None),
    );
}

fn make_path_way() -> Room {
    let mut actions = HashMap::new();

    actions.insert(Action::NONE, None);

    let mut exits = Vec::new();

    exits.push(make_exit(Direction::N, make_creek()));

    return Room::new(
        "Path".to_string(),
        "As you walk up the path the trees around seem to get thicker, taller and the area darker with shade. The path continues to go north.".to_string(),
        actions,
        exits,
        GoBack::new(true, None),
    );
}

fn make_creek() -> Room {
    let mut actions = HashMap::new();

    actions.insert(Action::NONE, None);

    let mut exits = Vec::new();

    exits.push(Exit {
        direction: Direction::NONE,
        room: None
    });

    return Room::new(
        "River".to_string(),
        r#"Continuing to follow the path, you come across a creek. The water is softly and quietly moving along its course. A prescence causes you to shudder and turn around. You see an old man standing behind you wearing a fedora red robes leaning on a staff. He looks at you for a moment before saying: "Hello there!""#.to_string(),
        actions,
        exits,
        GoBack::new(false, Some("You are engaged in a conversation with a strange old man. Don't be rude.".to_string())),
    );
}
