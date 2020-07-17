use std::vec::Vec;


/// Directions the player can move in.
#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    N, S, W, E, NONE, BACK
}

/// Acceptable Actions a player can take.
#[derive(Clone, Debug)]
pub enum Action {
    Look, Explore, NONE
}

/// Room deffinition.
///
/// The core aspects are what is the name, the description and finally:
/// what can I do? (Action)
/// Where can I go? (Exit)
///
/// Rooms are built start to finish. That is its a nested tree where you progress top
/// down through the various rooms. Rooms can branch off with Exits.
///
/// To visualize this imagine:
///
/// ```
///
/// room 1
///   |
///  Exits -> Room 2 or Room 3
///             |        |
///           Exits .. Exits ..
///
/// ```
///
///  All rooms must start from the rop down and based on the direction the plater takes
///  we then find that room in the vector of exits and load it.
///
///  Rooms do not have to link back to the previous room, because the world contains the current room
///  and the previous room.
#[derive(Clone, Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub actions: Vec<Action>,
    pub exits: Vec<Exit>,
}


impl Room {
    pub fn new(name: String, description: String, actions: Vec<Action>, exits: Vec<Exit>) -> Self {
        Room {
            name: name,
            description: description,
            actions: actions,
            exits: exits,
        }
    }

    pub fn describe(&self) -> &String {
        return &self.description;
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn exit(&self, direction: Direction) -> Option<Room> {

        for exit in &self.exits {
            if exit.direction == direction {

                let next_room = exit.room.clone();

                // I feel confident that if we fail here, someone messed up.
                // This would mean you have a room with a exit but no room for the player
                // to go too. So we should allow the system to panic.
                return Some(next_room.unwrap());
            }
        }


        return None;
    }
}

#[derive(Clone, Debug)]
pub struct Exit {
    pub direction: Direction,
    pub room: Option<Room>
}

pub fn make_exit(direction: Direction, room: Room) -> Exit {
    Exit {
        direction: direction,
        room: Some(room),
    }
}

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
