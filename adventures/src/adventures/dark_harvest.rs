use game::game::set_up_game::Game;

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

    while game.is_active() {


        game.set_active(false);
    }
}
