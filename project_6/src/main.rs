use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to read words from a file and validate lengths
fn read_words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut words = Vec::new();

    for line in reader.lines() {
        if let Ok(word_line) = line {
            let parts: Vec<&str> = word_line.trim().split_whitespace().collect();
            if parts.len() != 2 {
                continue; // Ignore lines that don't have exactly two elements
            }

            let expected_length: usize = parts[0].parse().unwrap_or(0);
            let actual_word = parts[1].to_lowercase();

            if actual_word.len() == expected_length {
                words.push(actual_word);
            }
        }
    }

    if words.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No valid words found in the file"));
    }

    Ok(words)
}

// Function to create a crossword layout (basic horizontal layout)
fn generate_crossword(words: &Vec<String>) -> Vec<Vec<char>> {
    let max_word_length = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let grid_size = max_word_length.max(words.len());

    let mut grid = vec![vec![' '; grid_size]; grid_size];

    for (i, word) in words.iter().enumerate() {
        if i < grid_size {
            for (j, c) in word.chars().enumerate() {
                grid[i][j] = c;
            }
        }
    }

    grid
}

// Function to print the crossword grid
fn print_crossword(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &ch in row {
            print!("{}", if ch == ' ' { '.' } else { ch });
        }
        println!();
    }
}

// Function to calculate the compactness score
fn calculate_compactness(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = if !grid.is_empty() { grid[0].len() } else { 0 };
    rows * cols
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <filename>");
        return;
    }

    let filename = &args[1];

    // Read words from the input file
    let words = match read_words_from_file(filename) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    // Generate crossword puzzle
    let crossword_grid = generate_crossword(&words);

    // Print the crossword puzzle
    println!("\nGenerated Crossword Puzzle:");
    print_crossword(&crossword_grid);

    // Calculate and display compactness score
    let compactness_score = calculate_compactness(&crossword_grid);
    println!("\nCompactness Score: {}", compactness_score);
}
