use std::char;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

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
                    'A' => 1, // center of target
                    'M' => 2, // need to find 2
                    'S' => 3, // need to find 2
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
            // print!("{cell_val}");
            if cell_val == 1 {
                match_count += find_matches_for_cell(&cells, &col, &row, &width, &height)
            }
        }
        // println!("");
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
) -> i32 {
    let mut match_count = 0;

    // check every possible direction for a match

    // needs to be off the edge by one
    if *x > 0 && *y > 0 && *x < (width - 1) && *y < (height - 1) {
        // need to find 2 M's (2's) that are on the same 'side' as each other
        // same for S's (3's)
        // check upper left cell and go from there
        let upper_left = cells[y - 1][x - 1];
        let upper_right = cells[y - 1][x + 1];
        let lower_left = cells[y + 1][x - 1];
        let lower_right = cells[y + 1][x + 1];
        if upper_right == lower_right && upper_left == lower_left {
            if (upper_right == 2 && upper_left == 3) || (upper_right == 3 && upper_left == 2) {
                match_count += 1;
            }
        }
        if upper_right == upper_left && lower_right == lower_left {
            if (upper_right == 2 && lower_right == 3) || (upper_right == 3 && lower_right == 2) {
                match_count += 1;
            }
        }
    }

    match_count
}
