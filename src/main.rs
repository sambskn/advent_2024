use std::char;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// use regex::Regex;

const STR_LENGTH: usize = 4;

fn read_lines_from_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // setup cells
    let mut cells: Vec<Vec<i8>> = vec![];

    println!("start!");
    if let Ok(lines) = read_lines_from_file(file_path) {
        for line in lines.flatten() {
            let mut row: Vec<i8> = vec![];
            for letter in line.chars() {
                // give each cell a value based on word we're looking for
                row.push(match letter {
                    'X' => 1,
                    'M' => 2,
                    'A' => 3,
                    'S' => 4,
                    _ => 0,
                })
            }
            cells.push(row);
        }
    }
    let mut match_count = 0;
    let height = cells.len();
    let width = cells[0].len();
    for row in 0..cells.len() {
        for col in 0..cells[row].len() {
            let cell_val = cells[row][col];
            // check every possible direction for match, if it's the start
            if cell_val == 1 {
                match_count += find_matches_for_cell(&cells, &col, &row, &width, &height, STR_LENGTH)
            }
        }
    }
    println!("total match count: {match_count}")
}

// only run for the start of matches plz
pub fn find_matches_for_cell(
    cells: &Vec<Vec<i8>>,
    x: &usize,
    y: &usize,
    width: &usize,
    height: &usize,
    str_length: usize
) -> i32 {
    let mut match_count = 0;
    
    // check every possible direction for a match
    
    // left to right
    if *x <= (width - str_length) {
        // check other vals
        if cells[*y][x + 1] == 2 && cells[*y][x + 2] == 3 && cells[*y][x + 3] == 4 {
            match_count += 1
        }
    }
    
    // right to left
    if *x >= (str_length - 1) {
        // check other vals
        if cells[*y][x - 1] == 2 && cells[*y][x - 2] == 3 && cells[*y][x - 3] == 4 {
            match_count += 1
        }
    }
    
    // down
    if *y <= (height - str_length) {
        // check other vals
        if cells[y + 1][*x] == 2 && cells[y + 2][*x] == 3 && cells[y + 3][*x] == 4 {
            match_count += 1
        }
    }
    
    // up
    if *y >= (str_length - 1) {
        // check other vals
        if cells[y - 1][*x] == 2 && cells[y - 2][*x] == 3 && cells[y - 3][*x] == 4 {
            match_count += 1
        }
    }
    
    // down right
    if *y >= (str_length - 1) && *x <= (width - str_length) {
        // check other vals
        if cells[y - 1][x + 1] == 2 && cells[y - 2][x + 2] == 3 && cells[y - 3][x + 3] == 4 {
            match_count += 1
        }
    }
    
    // up right
    if *y <= (height - str_length) && *x <= (width - str_length) {
        // check other vals
        if cells[y + 1][x + 1] == 2 && cells[y + 2][x + 2] == 3 && cells[y + 3][x + 3] == 4 {
            match_count += 1
        }
    }
    
    // down left
    if *y >= (str_length - 1) && *x >= (str_length - 1) {
        // check other vals
        if cells[y - 1][x - 1] == 2 && cells[y - 2][x - 2] == 3 && cells[y - 3][x - 3] == 4 {
            match_count += 1
        }
    }
    
    // up left
    if *y <= (height - str_length) && *x >= (str_length - 1) {
        // check other vals
        if cells[y + 1][x - 1] == 2 && cells[y + 2][x - 2] == 3 && cells[y + 3][x - 3] == 4 {
            match_count += 1
        }
    }
    
    
    match_count
}
