use std::collections::HashMap;
use std::vec::Vec;
use crate::core::process_call_backs::SimpleCallback;

/// Makes an adventure list of i32 => SimpleCallback for each item in the Vec passed in.
pub fn make_adventure_list(adventure_list: Vec<SimpleCallback>) -> HashMap<i32, SimpleCallback> {
    let mut adventures = HashMap::new();

    let mut count = 1;

    for adventure in adventure_list {
        adventures.insert(count, adventure);

        count = count + 1;
    }

    adventures
}
