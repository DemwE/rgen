use crate::args::RgenArgs;
use rand::Rng;
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn generate_passwords(args: RgenArgs) {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let special_characters = "!@#$%^&*()_-+=<>?";

    let mut rng = rand::thread_rng();
    let mut available_characters = characters.to_string();
    if args.special {
        available_characters.push_str(special_characters);
    }

    let count = args.count.unwrap_or(1);    // Use 1 as the default value
    let length = args.length.unwrap_or(16); // Use 16 as the default value

    let file_path = args.save.clone();
    let mut writer: Option<BufWriter<File>> = None;

    if let Some(file_path) = &file_path {
        let file = File::create(file_path).expect("Failed to create the output file");
        writer = Some(BufWriter::new(file));
    }

    for _ in 0..count {
        let password: String = (0..length)
            .map(|_| available_characters.chars().nth(rng.gen_range(0..available_characters.len())).unwrap())
            .collect();

        if let Some(ref mut writer) = writer {
            if args.raw {
                writeln!(writer, "{}", password).expect("Failed to write to file");
            } else {
                writeln!(writer, "Generated password: {}", password).expect("Failed to write to file");
            }
        } else {
            if args.raw {
                println!("{}", password);
            } else {
                println!("Generated password: {}", password);
            }
        }
    }

    if let Some(ref mut writer) = writer {
        writer.flush().expect("Failed to flush the file");
        println!("Passwords saved to: {}", file_path.unwrap());
    }
}
