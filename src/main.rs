use rand::seq::SliceRandom;
//use rand::Rng;
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashSet;

// Define command-line arguments using Clap
#[derive(Parser)]
#[command(author, version, about = "A secure password generator in Rust")]
struct Args {
    /// Length of the password
    #[arg(short = 'l', long = "length", default_value_t = 16)]
    length: usize,

    /// Verbose mode, prints password to terminal
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

// Define the character set without similar characters
const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ\
                         abcdefghijkmnpqrstuvwxyz\
                         23456789\
                         !@#$%^&*()_+-=[]{}|;:,.<>?";

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();

    loop {
        // Shuffle CHARSET to ensure random order
        let password: Vec<char> = CHARSET
            .choose_multiple(&mut rng, length)
            .cloned()
            .map(|c| c as char)
            .collect();

        // Check for uniqueness, no duplicates, and no sequential characters
        if has_unique_characters(&password)
            && !has_sequential_characters(&password)
        {
            return password.into_iter().collect();
        }
    }
}

fn has_unique_characters(password: &[char]) -> bool {
    let unique_chars: HashSet<&char> = password.iter().collect();
    unique_chars.len() == password.len()
}

fn has_sequential_characters(password: &[char]) -> bool {
    for i in 0..password.len() - 1 {
        if password[i] as u8 + 1 == password[i + 1] as u8 {
            return true;
        }
    }
    false
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Generate the password
    let password = generate_password(args.length);

    // Copy password to clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to access clipboard");
    ctx.set_contents(password.clone()).expect("Failed to set clipboard content");

    // Output password if verbose mode is enabled
    if args.verbose {
        println!("{}", password);
    }
}

