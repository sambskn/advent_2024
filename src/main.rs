use std::char;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Loading file input @ {file_path}...");
    let input_contents = fs::read_to_string(file_path).expect("Couldn't read file");
    let input_number_strings: Vec<&str> =
        input_contents.as_str().split(char::is_whitespace).collect();
    let mut list_1 = vec![];
    let mut list_2 = vec![];
    for num_str in input_number_strings.iter() {
        let str_length = num_str.len();
        if str_length > 0 {
            let num: u32 = (**num_str).parse().expect("input should be a number");
            if list_1.len() > list_2.len() {
                let idx = list_2.partition_point(|&x| x <= num);
                list_2.insert(idx, num);
            } else {
                let idx = list_1.partition_point(|&x| x <= num);
                list_1.insert(idx, num);
            }
        }
    }
    let mut sim_score = 0;
    for idx in 0..list_1.len() {
        let list_1_val = list_1[idx];
        let mut count = 0;
        for list_2_val in list_2.iter() {
            if *list_2_val == list_1_val {
                count += 1;
            }
        }
        sim_score += count * list_1_val;
    }
    println!("sim score: {sim_score}")
}
