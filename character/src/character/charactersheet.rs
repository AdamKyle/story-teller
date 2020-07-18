use std::option::Option;
use std::io;
use std::io::Write;
use std::process;
use rand::Rng;



/// Defeinition of a character in the game.
///
/// All characters must have a name, other options can be
/// optional.
#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub stats: Option<Stats>,
    pub class: Option<Class>,
    pub race: Option<Race>,
}

/// Stats of a character.
#[derive(Clone, Debug)]
pub struct Stats {
    pub int: i32,
    pub str: i32,
    pub dex: i32,
    pub chr: i32,
    pub dur: i32,
}

/// Implementation of stats.
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

/// Character race.
#[derive(Clone, Debug)]
pub struct Race {
    pub name: String,
}

/// Implementation of race.
impl Race {
    pub fn new(name: String) -> Self {
        Race {
            name: name
        }
    }
}

/// Character class.
#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
}

/// Implementation of class.
impl Class {
    pub fn new(name: String) -> Self {
        Class {
            name: name
        }
    }
}

/// Build a character based off name.
///
/// This is great for when we want to get going. All we care
/// about is the name at this point.
pub fn build_character(name: String) -> Character {
    Character {
        name: name,
        race: None,
        class: None,
        stats: None
    }
}

/// Create the stats for the character and return that character.
///
/// We generate a set of intgers based on the number of stats for the character.
/// from there we can spit that out and then enter a loop asking you what you wnat to do.
/// repeat till valid input.
///
/// If we encounter re-roll, its the same process all over again with a new set of random numbers.
///
/// All stats are considered 3d6. Your lowest possible value is 3 and the highest beign 18.
pub fn create_stats(character: Character) -> Character {
    let mut stats: Vec<i32> = Vec::new();

    for _i in 0..5 {
        stats.push(rand::thread_rng().gen_range(3,18));
    }

    println!("\nRolled stats:");
    println!("===============");

    let mut count = 0;
    let stat_names = vec!["str", "int", "dex", "chr", "dur"];

    for stat in &stats {

        println!("{}: {}", stat_names[count], stat);
        count = count + 1;
    }

    println!("===============");
    println!("What would you like to do? (You can type: accept, re-roll, explain or quit)");
    println!("\n");

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
        } else if input.to_string() == "re-roll".to_string() {

            stats = Vec::new();

            for _i in 0..5 {
                stats.push(rand::thread_rng().gen_range(1,18));
            }

            println!("\nRolled stats:");
            println!("===============");

            let mut count = 0;

            for stat in &stats {

                println!("{}: {}", stat_names[count], stat);
                count = count + 1;
            }

            println!("===============");
            println!("What would you like to do? (You can type: accept, re-roll, explain or quit)");
            println!("\n");

        } else if input.to_string() == "accept".to_string() {
            done = true;

            let stats = accept_stats(&stats);

            character.stats = Some(stats);
        } else if input.to_string() == "quit".to_string() {
            println!("Bye now!");
            process::exit(1);
        }
    }

    return character;
}

fn accept_stats(stats: &Vec<i32>) -> Stats {
    println!("\nAccepted stats:, {:?}", stats);

    return Stats::new(stats[0], stats[1], stats[2], stats[3], stats[4]);
}
