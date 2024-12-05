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

    let mut reading_rules = true;

    let mut rules: Vec<Rule> = vec![];
    let mut valid_orderings: Vec<Vec<String>> = vec![];
    let mut fixed_orderings: Vec<Vec<String>> = vec![];

    println!("start!");
    if let Ok(lines) = read_lines_from_file(file_path) {
        for line in lines.flatten() {
            let line_len = line.len();
            if line_len == 0 {
                reading_rules = false;
            } else {
                if reading_rules {
                    // handle ordering rule
                    let parts: Vec<&str> = line.split("|").collect();
                    rules.push(Rule {
                        before: parts[0].to_string(),
                        after: parts[1].to_string(),
                    });
                } else {
                    // hanlde ordering
                    let parts: Vec<&str> = line.split(",").collect();
                    let mut ordering: Vec<String> = vec![];
                    for part in parts {
                        ordering.push(part.to_string());
                    }
                    if is_valid(&ordering, &rules) {
                        valid_orderings.push(ordering);
                    } else {
                        fixed_orderings.push(fix_ordering(&ordering, &rules))
                    }
                }
            }
        }
        let valid_count = valid_orderings.len();
        println!("valid count: {valid_count}");

        // get middle vals
        let mut total = 0;
        for ordering in valid_orderings.iter() {
            let ordering_middle = (ordering.len() - 1) / 2;
            let num: u32 = ordering[ordering_middle].parse().unwrap();
            total += num;
        }
        println!("total of middle pages: {total}");

        total = 0;
        for ordering in fixed_orderings.iter() {
            let ordering_middle = (ordering.len() - 1) / 2;
            let num: u32 = ordering[ordering_middle].parse().unwrap();
            total += num;
        }
        println!("total of middle pages: {total}");
    }
}

pub struct Rule {
    pub before: String,
    pub after: String,
}

pub fn is_valid(ordering: &Vec<String>, rules: &Vec<Rule>) -> bool {
    for idx in 0..ordering.len() {
        for rule in rules.iter() {
            if rule.after == ordering[idx] {
                // check if there's any val after thats supposed to be before
                for sub_idx in (idx + 1)..ordering.len() {
                    if ordering[sub_idx] == rule.before {
                        return false;
                    }
                }
            }
        }
    }
    true
}

pub fn fix_ordering(ordering: &Vec<String>, rules: &Vec<Rule>) -> Vec<String> {
    let mut new_ordering = ordering.clone();
    while !is_valid(&new_ordering, rules) {
        for idx in 0..ordering.len() {
            for rule in rules.iter() {
                if rule.after == new_ordering[idx] {
                    // check if there's any val after thats supposed to be before
                    for sub_idx in (idx + 1)..ordering.len() {
                        if new_ordering[sub_idx] == rule.before {
                            println!("broken rule before {} after {}", rule.before, rule.after);
                            let popped = new_ordering.remove(idx);
                            new_ordering.push(popped);
                        }
                    }
                }
            }
        }
    }
    new_ordering
}
