mod game;

use character::charactersheet::Character;

pub use crate::game::set_up_game::Game;

/// Create a game with a Character.
pub fn create_game(character: Character) -> Game {
    Game  {
        active: true,
        game_character: character,
    }
}
