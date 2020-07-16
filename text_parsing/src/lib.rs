use std::vec::Vec;

pub fn parse_input(input: String) {
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        println!("Invalid input.");
    }

    println!("{:?}", words);
}
