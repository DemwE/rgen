use rand::seq::SliceRandom;
use crate::args;
use std::include_str;

pub fn generator(args: &args::RgenArgs) -> String
{
    let mut rng = rand::thread_rng();

    let passphrases = if args.custom_passphrase_list.is_some() {
        match std::fs::read_to_string(args.custom_passphrase_list.as_ref().unwrap()) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }
    } else {
        include_str!("passphrases.txt").to_string()
    };
    
    let available_passphrases = passphrases.split("\n").collect::<Vec<&str>>();

    let count = args.passphrase_count.unwrap_or(3);    // Use 3 as the default value

    let password: Vec<String> = (0..count)
            .map(|_| available_passphrases.choose(&mut rng).unwrap().to_string())
            .collect();

    return password.join("-");
}
