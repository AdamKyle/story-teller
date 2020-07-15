use character::charactersheet::Character;

/// Game structure.
///
/// Contains information about the game including its status and the
/// character.
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
    pub fn set_acive(&mut self, active: bool) {
        self.active = active;
    }

    /// Get the character object.
    pub fn get_character(&self) -> &Character {
        return &self.game_character;
    }

    /// Set the character.
    pub fn update_character(&mut self, character: Character) {
        self.game_character = character;
    }
}
