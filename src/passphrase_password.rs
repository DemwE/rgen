use rand::seq::SliceRandom;
use crate::args;
use std::include_str;

pub fn generator(args: &args::RgenArgs) -> String
{
    let mut rng = rand::thread_rng();

    let passphrases = include_str!("passphrases.txt");
    let available_passphrases = passphrases.split("\n").collect::<Vec<&str>>();

    let count = args.passphrase_count.unwrap_or(3);    // Use 3 as the default value

    let password: Vec<String> = (0..count)
            .map(|_| available_passphrases.choose(&mut rng).unwrap().to_string())
            .collect();

    return password.join("-");
}
