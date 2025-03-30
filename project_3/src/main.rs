use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const GRID_SIZE: usize = 20; // Set a fixed grid size for simplicity

type Grid = Vec<Vec<char>>;

fn initialize_grid(size: usize) -> Grid {
    vec![vec![' '; size]; size] // Fill grid with empty spaces
}

fn read_words(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut words = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let word = parts[0].to_lowercase(); // Read first word in the line
        words.push(word);
    }
    
    Ok(words)
}

fn place_word(grid: &mut Grid, word: &str, row: usize, col: usize, vertical: bool) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();

    if vertical {
        if row + len > GRID_SIZE {
            return false; // Check bounds
        }
        for i in 0..len {
            if grid[row + i][col] != ' ' && grid[row + i][col] != chars[i] {
                return false; // Conflict with another word
            }
        }
        for i in 0..len {
            grid[row + i][col] = chars[i];
        }
    } else {
        if col + len > GRID_SIZE {
            return false;
        }
        for i in 0..len {
            if grid[row][col + i] != ' ' && grid[row][col + i] != chars[i] {
                return false;
            }
        }
        for i in 0..len {
            grid[row][col + i] = chars[i];
        }
    }
    true
}

fn print_grid(grid: &Grid) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn calculate_compactness(grid: &Grid) -> usize {
    let mut min_row = GRID_SIZE;
    let mut max_row = 0;
    let mut min_col = GRID_SIZE;
    let mut max_col = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch != ' ' {
                if r < min_row {
                    min_row = r;
                }
                if r > max_row {
                    max_row = r;
                }
                if c < min_col {
                    min_col = c;
                }
                if c > max_col {
                    max_col = c;
                }
            }
        }
    }

    if min_row > max_row || min_col > max_col {
        return 0; // No words placed
    }

    (max_row - min_row + 1) * (max_col - min_col + 1)
}

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
    let mut row = 5;
    let mut col = 5;
    let mut vertical = false;

    for word in words {
        if !place_word(&mut grid, &word, row, col, vertical) {
            eprintln!("Could not place word: {}", word);
        }
        vertical = !vertical;
        row += 2;
        col += 2;
    }
    
    print_grid(&grid);
    let compactness = calculate_compactness(&grid);
    println!("Compactness Score: {}", compactness);
}
