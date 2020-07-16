use std::vec::Vec;

#[derive(Clone, Debug)]
pub enum Direction {
    N, S, W, E, NONE
}

#[derive(Clone, Debug)]
pub enum Action {
    Look, Explore, NONE
}

#[derive(Clone, Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub actions: Vec<Action>,
    pub exits: Vec<Exit>
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

#[derive(Debug)]
pub struct World {
    pub room: Room
}

impl World {

    pub fn new(room: Room) -> Self{
        World {
            room: room
        }
    }

    pub fn get_base_room(&self) -> &Room {
        return &self.room
    }
}
