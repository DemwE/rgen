use rand::Rng;
use crate::args;

pub fn generator(args: &args::RgenArgs) -> String
{
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let special_characters = "!@#$%^&*()_-+=<>?";

    let mut rng = rand::thread_rng();
    let mut available_characters = characters.to_string();
    if args.special {
        available_characters.push_str(special_characters);
    }

    let length = args.length.unwrap_or(16); // Use 16 as the default value

    let password: String = (0..length)
            .map(|_| available_characters.chars().nth(rng.gen_range(0..available_characters.len())).unwrap())
            .collect();

    return password;
}