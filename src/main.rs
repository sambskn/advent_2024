use std::char;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use regex::Regex;

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
    let commands_regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let numbers_regex = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut total = 0;
    let mut enabled = true;
    println!("start!");
    if let Ok(lines) = read_lines_from_file(file_path) {
        for line in lines.flatten() {
            let strs: Vec<&str> = commands_regex.find_iter(line.as_str()).map(|m| m.as_str()).collect();
            for mul_str in strs.into_iter() {
                if mul_str.contains("do()") {
                    println!("on");
                    enabled = true;
                } else if mul_str.contains("don't()") {
                    println!("off");
                    enabled = false;
                } else if mul_str.contains("mul") && enabled {
                    let num_strs: Vec<&str> =
                        numbers_regex.find_iter(mul_str).map(|m| m.as_str()).collect();
                    let x: u32 = num_strs[0].parse().expect("no number???");
                    let y: u32 = num_strs[1].parse().expect("no number???");
                    total += x * y;
                    println!("bump");
                }
            }
        }
    }
    println!("total: {total}");
}
