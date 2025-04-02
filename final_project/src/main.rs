use std::env; // Import environment module for reading command-line arguments
use std::fs::File; // Import File module to handle file operations
use std::io::{self, BufRead}; // Import io module for reading the file
use std::collections::HashSet; // Import HashSet to store words

const GRID_SIZE: usize = 15; // Define the crossword grid size

type Grid = Vec<Vec<char>>; 

/// Initializes a GRID_SIZE x GRID_SIZE crossword grid filled with empty spaces.
fn initialize_grid(size: usize) -> Grid {
    vec![vec![' '; size]; size]
}

/// Reads words from a file, extracts the second column, converts to lowercase, and removes duplicates.
fn read_words(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut words = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            let word = parts[1].to_lowercase();
            words.insert(word);
        }
    }

    Ok(words.into_iter().collect()) // Convert the HashSet into a Vec
}

/// Tries to place the word at a specific position (either vertically or horizontally).
fn place_word(grid: &mut Grid, word: &str, row: usize, col: usize, vertical: bool) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();

    if vertical {
        if row + len > GRID_SIZE {
            return false; 
        }
        for (i, &ch) in chars.iter().enumerate() {
            if grid[row + i][col] != ' ' && grid[row + i][col] != ch {
                return false;
            }
        }
        for (i, &ch) in chars.iter().enumerate() {
            grid[row + i][col] = ch;
        }
    } else {
        if col + len > GRID_SIZE {
            return false; 
        }
        for (i, &ch) in chars.iter().enumerate() {
            if grid[row][col + i] != ' ' && grid[row][col + i] != ch {
                return false; 
            }
        }
        for (i, &ch) in chars.iter().enumerate() {
            grid[row][col + i] = ch; 
        }
    }
    true 
}

/// Prints the crossword grid, keeping empty spaces blank.
fn print_grid(grid: &Grid) {
    println!("\nCrossword Puzzle:");
    println!("{}", "-".repeat(GRID_SIZE * 2));

    for row in grid {
        let row_string: String = row.iter().collect();
        println!("{}", row_string); 
    }

    println!("{}", "-".repeat(GRID_SIZE * 2));
}

/// Computes the compactness score based on the smallest bounding rectangle.
fn calculate_compactness(grid: &Grid) -> usize {
    let mut min_row = GRID_SIZE;
    let mut max_row = 0;
    let mut min_col = GRID_SIZE;
    let mut max_col = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch != ' ' {
                min_row = min_row.min(r);
                max_row = max_row.max(r);
                min_col = min_col.min(c);
                max_col = max_col.max(c);
            }
        }
    }

    if min_row > max_row || min_col > max_col {
        return 0; // No words placed
    }

    (max_row - min_row + 1) * (max_col - min_col + 1) // Compactness score calculation
}

/// Tries to place words near previously placed words for better compactness.
fn place_word_near(grid: &mut Grid, word: &str) -> bool {
    // Try placing the word at the intersection with any previously placed word
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            // Try placing the word vertically and horizontally from this position
            if place_word(grid, word, r, c, true) || place_word(grid, word, r, c, false) {
                return true;
            }
        }
    }
    false // Unable to place word near others
}

/// Main function: Reads words, places them in the crossword grid, and calculates compactness score.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let words = match read_words(filename) {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut grid = initialize_grid(GRID_SIZE);

    // Place each word either near an already placed word or randomly in the grid
    for word in words {
        if !place_word_near(&mut grid, &word) {
            let mut placed = false;
            for r in 0..GRID_SIZE {
                for c in 0..GRID_SIZE {
                    if place_word(&mut grid, &word, r, c, true) || place_word(&mut grid, &word, r, c, false) {
                        placed = true;
                        break;
                    }
                }
                if placed {
                    break;
                }
            }
            // If still not placed, report failure
            if !placed {
                eprintln!("Could not place word: {}", word);
            }
        }
    }

    print_grid(&grid); // Display the crossword
    let compactness = calculate_compactness(&grid);
    println!("Compactness Score: {}", compactness);
}
