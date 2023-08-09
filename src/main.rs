mod args;

use clap::Parser;
use args::RgenArgs;
use rand::Rng;

fn main() {
    // Parse the command line arguments
    let args = RgenArgs::parse();

    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let special_characters = "!@#$%^&*()_-+=<>?";

    let mut rng = rand::thread_rng();
    let mut available_characters = characters.to_string();
    if args.special {
        available_characters.push_str(special_characters);
    }

    let count = args.count.unwrap_or(1);    // Use 1 as the default value
    let length = args.length.unwrap_or(16); // Use 16 as the default value

    for _ in 0..count {
        let password: String = (0..length)
            .map(|_| available_characters.chars().nth(rng.gen_range(0..available_characters.len())).unwrap())
            .collect();

        if args.raw {
            println!("{}", password);
            continue;
        }
        else {
            println!("Generated password: {}", password);
        }
    }
}
