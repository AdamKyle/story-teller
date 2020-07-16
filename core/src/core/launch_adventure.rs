use std::collections::HashMap;
use crate::core::process_call_backs::SimpleCallback;

/// Launches the adventure based on the number entered.
///
/// This is responsible for starting the adventure based on the user input.
///
/// Will panic if we cant find an adventure in the hash map based on the input.
pub fn start_adventure(mut adventure: String, adventures: HashMap<i32, SimpleCallback>) {
    if adventure.ends_with('\n') {
        adventure.pop();
    }

    for (number, adv) in adventures {
        if number.to_string() == adventure {
            adv.process();
        } else {
            panic!("Could not load adventure that doesn't exist. Please consider this a bug.");
        }
    }
}
