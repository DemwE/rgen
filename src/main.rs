mod args;
mod passphrase_password;
mod random_password;

use args::RgenArgs;
use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Parse the command line arguments
    let args = RgenArgs::parse();
    let file_path = args.save.clone();
    let mut writer: Option<BufWriter<File>> = None;
    let count = args.count.unwrap_or(1); // Use 1 as the default value

    if let Some(file_path) = &file_path {
        let file = File::create(file_path).expect("Failed to create the output file");
        writer = Some(BufWriter::new(file));
    }

    // let mut password = "si";
    for _ in 0..count {
        let password = if args.passphrase {
            passphrase_password::generator(&args)
            
        } else {
            random_password::generator(&args)            
        };
        if let Some(ref mut writer) = writer {
            if args.raw {
                writeln!(writer, "{}", password).expect("Failed to write to file");
            } else {
                writeln!(writer, "Generated password: {}", password)
                    .expect("Failed to write to file");
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
