use std::option::Option;
use std::io;
use std::io::Write;
use std::process;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub stats: Option<Stats>,
    pub class: Option<Class>,
    pub race: Option<Race>,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub int: i32,
    pub str: i32,
    pub dex: i32,
    pub chr: i32,
    pub dur: i32,
}

impl Stats {
    pub fn new(str: i32, int: i32, dex: i32, chr: i32, dur: i32) -> Self {
        Stats {
            int: int,
            str: str,
            dex: dex,
            chr: chr,
            dur: dur,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Race {
    pub name: String,
}

impl Race {
    pub fn new(name: String) -> Self {
        Race {
            name: name
        }
    }
}

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class {
            name: name
        }
    }
}

pub fn build_character(name: String) -> Character {
    Character {
        name: name,
        race: None,
        class: None,
        stats: None
    }
}

pub fn create_stats(character: Character) -> Character {
    let mut stats: Vec<i32> = Vec::new();

    for _i in 0..5 {
        stats.push(rand::thread_rng().gen_range(1,18));
    }

    println!("\nRolled stats: {0:?}", stats);
    println!("All stats are rolled on a 3d6 and corelate to: str, int, dex, chr, dur respectivly.");
    println!("What would you like to do? (You can type: accept, re-roll, explain or quit)");

    let mut done = false;

    let mut character = character;

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
            println!("Invalid input. Try again.");
        }

        if input.to_string() == "re-roll".to_string() {

            stats = Vec::new();

            for _i in 0..5 {
                stats.push(rand::thread_rng().gen_range(1,18));
            }

            println!("\nRolled stats: {0:?}", stats);
            println!("What would you like to do? (You can type: accept, re-roll, explain or quit)");

        } else if input.to_string() == "accept".to_string() {
            done = true;

            let stats = accept_stats(&stats);

            character.stats = Some(stats);
        } else if input.to_string() == "quit".to_string() {
            println!("Bye now!");
            process::exit(1);
        } else {
            println!("Invalid input. Try again.");
        }
    }

    return character;
}

fn accept_stats(stats: &Vec<i32>) -> Stats {
    println!("\nAccepted stats:, {:?}", stats);

    return Stats::new(stats[0], stats[1], stats[2], stats[3], stats[4]);
}
