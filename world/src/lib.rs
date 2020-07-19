mod world;

pub use crate::world::actions;
pub use crate::world::conversation;
pub use crate::world::room;
pub use crate::world::person;

use crate::world::room::Room;

/// The core world sturcture that contains the room.
///
/// The room is the base room that then contains exits in to other rooms
/// that can then branch off.
///
/// To get around moving back to the previous room the world will contain
/// a previous room and a current_room, which is the room you were in and the room you are in
/// respectivly.
#[derive(Debug)]
pub struct World {
    pub room: Room,
    pub current_room: Option<Room>,
    pub previous_room: Option<Room>,
}

impl World {

    /// Create a new world with the base room.
    pub fn new(room: Room) -> Self{
        World {
            room: room,
            current_room: None,
            previous_room: None,
        }
    }

    /// Gets the base room.
    pub fn get_base_room(&self) -> &Room {
        return &self.room
    }

    pub fn set_current_room(mut self, current_room: Room) -> World {
        self.current_room = Some(current_room);

        return self
    }
}
