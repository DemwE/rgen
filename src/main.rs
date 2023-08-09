mod args;
mod password_generator;

use clap::Parser;
use args::RgenArgs;
use password_generator::generate_passwords;

fn main() {
    // Parse the command line arguments
    let args = RgenArgs::parse();

    generate_passwords(args);
}
