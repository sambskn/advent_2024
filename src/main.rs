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
    println!("Loading file input @ {file_path}...");
    let mut passing_count = 0;
    if let Ok(lines) = read_lines_from_file(file_path) {
        for line in lines.flatten() {
            let mut test_results = vec![];
            let num_strs: Vec<&str> = line.split(char::is_whitespace).collect();
            for num_str in num_strs.iter() {
                let num: u32 = num_str.parse().expect("expected a number");
                test_results.push(num);
            }
            if test_level_set(&test_results) {
                passing_count += 1;
            }
        }
    }
    println!("passing count: {passing_count}");
}

pub fn test_level_set(levels: &Vec<u32>) -> bool {
    if is_level_set_valid(levels) {
        return true;
    }
    // try removing each element
    for level_idx in 0..levels.len() {
        let mut trimmed = levels.clone();
        trimmed.remove(level_idx);
        if is_level_set_valid(&trimmed) {
            return true;
        }
    }
    false
}

pub fn is_level_set_valid(levels: &Vec<u32>) -> bool {
    let mut diffs = vec![];
    for level_idx in 0..(levels.len() - 1) {
        let diff = levels[level_idx + 1] as i32 - levels[level_idx] as i32;
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
        diffs.push(diff);
    }
    let is_increasing = diffs[0] > 0;
    for diff in diffs.iter() {
        let increasing_diff = *diff > 0;
        if increasing_diff != is_increasing {
            return false;
        }
    }
    true
}
