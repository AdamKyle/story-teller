use std::collections::HashMap;
use crate::core::process_call_backs::SimpleCallback;

/// Launches the adventure based on the number entered.
pub fn start_adventure(mut adventure: String, adventures: HashMap<i32, SimpleCallback>) {
    if adventure.ends_with('\n') {
        adventure.pop();
    }

    for (number, adv) in adventures {
        if number.to_string() == adventure {
            adv.process();
        } else {
            println!("Invalid input.");
        }
    }
}
