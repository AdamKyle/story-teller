use std::vec::Vec;
use std::collections::HashMap;


/// Directions the player can move in.
#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    N, S, W, E, NONE, BACK
}

/// Acceptable Actions a player can take.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Look, Explore, NONE
}

/// Do something on action
///
/// Actions are what the player can do.
#[derive(Clone, Debug)]
pub struct OnAction {
    pub on_action: String,
}

impl OnAction {

    pub fn new(on_action: String) -> Self {
        OnAction {
            on_action: on_action,
        }
    }

    pub fn do_action(&mut self) {
        println!("{}", self.on_action);
    }
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
/// All rooms must start from the rop down and based on the direction the plater takes
/// we then find that room in the vector of exits and load it.
///
/// Rooms do not have to link back to the previous room, because the world contains the current room
/// and the previous room.
///
/// When a player does an action on a room we want to process that action, to do this we take in the action that
/// the user types, for example: "look" or "explore", from there we loop over the rooms actions looking for
/// that action. Because OnAction is an Option its suggested that you only set it to None, if the Action is also
/// none. That indicates the layer has no actions in this room.
///
/// Sometimes a room needs to not allow the player to be able to go back. We allow you to set up
/// a room that allows the player go back or not, if you say false to that and the player types "go back"
/// The playr will then be given a reason that you specified.
///
#[derive(Clone, Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub actions: HashMap<Action, Option<OnAction>>,
    pub exits: Vec<Exit>,
    pub go_back: GoBack,
}

#[derive(Clone, Debug)]
pub struct GoBack {
    pub can_go_back: bool,
    pub reason: Option<String>,
}

impl GoBack {

    pub fn new(can_go_back: bool, reason: Option<String>) -> Self {
        GoBack {
            can_go_back: can_go_back,
            reason: reason,
        }
    }
}


impl Room {
    pub fn new(name: String, description: String, actions: HashMap<Action, Option<OnAction>>, exits: Vec<Exit>, go_back: GoBack) -> Self {
        Room {
            name: name,
            description: description,
            actions: actions,
            exits: exits,
            go_back: go_back
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

    pub fn do_action(&self, action: Action) {
        let actions = self.actions.clone();

        for (room_action, on_action) in actions {
            if room_action == action && on_action.is_some() {
                on_action.clone().unwrap().do_action();
                return;
            }
        }

        println!("Cannot do that action in this area.");
    }
}

/// Handels the rooms exit.
///
/// Because rooms are built in a top down fashion, we have take in the direction and the
/// room, which is an Option. we will panic if you have a direction but no room for the player to exit
/// out too. The only time room should be None is if there is no exit from this room.
///
/// Rooms do not need to define exits going backwards. We assume that you can always go back
/// to the previous room.
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
