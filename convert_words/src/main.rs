use std::env; // Import the environment module to read command-line arguments
use std::fs::File; // Import the File module to enable opening the file
use std::io::{self, BufRead}; // Import the io module for input and output operations. Import BufRead module for reading lines from a file efficiently
use std::path::Path; // Import the Path module. This module provides functionality for working with file and directory paths in a platform-independent way.

// Function to read words from a file and convert them to lowercase
fn read_words(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut words = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let word = parts[1].to_lowercase(); // Convert to lowercase
        words.push(word);
    }
    
    Ok(words)
}

fn main() {
    // Get the file name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    
    match read_words(filename) {
        Ok(words) => {
            println!("Words read from file: {:?}", words);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}