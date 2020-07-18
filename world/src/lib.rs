use std::io;
use std::io::Write;
use std::vec::Vec;
use std::collections::HashMap;
use rand::Rng;
use core::text_handeling::unwrap_str;

/// Directions the player can move in.
#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    N, S, W, E, NONE, BACK
}

/// Acceptable Actions a player can take.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Look, Explore, NONE, Talk
}

/// Do something on action
///
/// Actions are what the player can do.
///
/// the on_action is what happens when the action is succesffull.
/// the action is only successful if the dc_check passes.
///
/// If the Action does not have a dc_check then we should just do the on_action.
#[derive(Clone, Debug)]
pub struct OnAction {
    pub on_action: String,
    pub dc_check: Option<i32>,
}

impl OnAction {

    pub fn new(on_action: String, dc_check: Option<i32>) -> Self {
        OnAction {
            on_action: on_action,
            dc_check: dc_check,
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
    pub conversation: Option<Converse>,
}

#[derive(Clone, Debug)]
pub struct Converse {
    pub line: String,
    pub choices: Option<Vec<Choices>>,
}

impl Converse {

    pub fn new(line: String, choices: Option<Vec<Choices>>) -> Self {
        Converse {
            line: line,
            choices: choices,
        }
    }

    pub fn process_conversation(&mut self) {

        if !self.choices.clone().is_some() {
            return;
        }

        let choices = self.choices.clone().unwrap();

        let mut options = HashMap::new();

        options.insert(0, None);

        let mut count: i32 = 1;

        println!("\n===== [Choices] =====");

        for choice in &choices {
            options.insert(count, Some(choice.clone().choice));

            print!("{}) {}", count, choice.clone().choice);

            count = count + 1;
        }

        println!("\n=====================");
        println!("Choose one by typing the number or q, quit or exit to leave the conversation.");
        println!("\n");

        let mut done: bool = false;

        let mut next: Option<Converse> = None;

        while !done {
            print!("> ");

            io::stdout().flush().expect("Error flushing stdout!");

            let mut input = String::new();

            io::stdin().read_line(&mut input)
                       .expect("Error reading stdin!");

            if input.ends_with('\n') {
               input.pop();
            }

            let words: Vec<&str> = input.split_whitespace().collect();

            if words.is_empty() {
                println!("Invalid input.");
            } else {
                let mut result: Vec<String> = Vec::new();

                for word in words {
                    result.push(word.to_string());
                }

                if self.parse_quit(result.clone()) {
                    done = true;
                }

                let choice_selection = self.parse_choice_input(result.clone(), options.clone(), choices.clone());

                if choice_selection.is_some() {
                    next = choice_selection;
                    done = true;
                }
            }
        }

        println!("{}", next.clone().unwrap().line);
        next.clone().unwrap().process_conversation();
    }

    fn parse_choice_input(&mut self, words: Vec<String>, options: HashMap<i32, Option<String>>, choices: Vec<Choices>) -> Option<Converse> {
        let mut words = words.iter();

        let input = unwrap_str(words.next());

        match input.parse::<i32>() {
            Ok(n) => {
                if options.contains_key(&n) {
                    // You can't have a choice that leads to no convo. So if this then
                    // explodses thats on you for not supplying a a conversdation to a choice option
                    return Some(choices[n as usize - 1].next.clone().unwrap());
                } else {
                    println!("Not a valid choice.");

                    return None;
                }
            },
            Err(_e) => {
                println!("Not a valid choice.");

                return None;
            }
        }
    }

    fn parse_quit(&mut self, words: Vec<String>) -> bool {
        let mut words = words.iter();

        let command = unwrap_str(words.next());

        match command {
            "quit" | "q" | "exit" => {
                println!("You abruptly left the conversation.");
                return true;
            },
            _ => {
                return false;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Choices {
    pub choice: String,
    pub next: Option<Converse>,
}

impl Choices {

    pub fn new(choice: String, next: Option<Converse>) -> Self {
        Choices {
            choice: choice,
            next: next,
        }
    }
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
    pub fn new(name: String, description: String, actions: HashMap<Action, Option<OnAction>>, exits: Vec<Exit>, go_back: GoBack, conversation: Option<Converse>) -> Self {
        Room {
            name: name,
            description: description,
            actions: actions,
            exits: exits,
            go_back: go_back,
            conversation: conversation,
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

    pub fn do_action(&self, action: Action, bonus: i32) {
        let actions = self.actions.clone();

        for (room_action, on_action) in actions {
            if room_action == action && on_action.is_some() {

                if on_action.clone().unwrap().dc_check.is_some() {
                    let roll = rand::thread_rng().gen_range(1,20) + bonus;

                    let dc_check = on_action.clone().unwrap().dc_check.unwrap();

                    if roll > dc_check {
                        println!("Upon your roll of a: {}", roll);
                        on_action.clone().unwrap().do_action()
                    } else {
                        println!("Failed to do the action. You failed the DC check: {}. You can try again.", roll);
                    }
                }

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
