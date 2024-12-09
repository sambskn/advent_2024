use std::collections::HashSet;
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

    let mut cells = vec![];
    let mut start_idx = 0;
    let mut start_dir = StartDir::None;
    let mut height = 0;
    let mut width = 0;
    println!("start!");
    if let Ok(lines) = read_lines_from_file(file_path) {
        let mut idx = 0;
        for line in lines.flatten() {
            let line_str = line.as_str();
            width = line_str.len();
            for line_char in line_str.chars() {
                match line_char {
                    '.' => cells.push(0),
                    '#' => cells.push(1),
                    '^' => {
                        cells.push(0);
                        start_idx = idx;
                        start_dir = StartDir::Up;
                    }
                    '>' => {
                        cells.push(0);
                        start_idx = idx;
                        start_dir = StartDir::Right;
                    }
                    '<' => {
                        cells.push(0);
                        start_idx = idx;
                        start_dir = StartDir::Left;
                    }
                    'v' => {
                        cells.push(0);
                        start_idx = idx;
                        start_dir = StartDir::Down;
                    }
                    _ => {}
                }
                idx += 1;
            }
            height += 1;
        }
    }

    let mut location_total = 0;
    let cell_count = cells.len();
    println!("total cells: {cell_count}");
    for cell_idx in 0..cells.len() {
        let percent_done = (cell_idx as f32 / cell_count as f32) * 100.0;
        println!("{percent_done}%   --  count: {location_total}");
        if cells[cell_idx] == 0 {
            let mut new_cells = cells.clone();
            new_cells[cell_idx] = 1;
            let steps_to_exit = traverse_grid(&new_cells, start_idx, start_dir, width, height);
            if steps_to_exit == 0 {
                location_total += 1;
            }
        }
    }
    println!("spots {location_total}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StartDir {
    Up,
    Down,
    Left,
    Right,
    None,
}

const MAX_STEPS: u32 = 99999;

pub fn traverse_grid(
    cells: &Vec<u8>,
    start_idx: usize,
    start_dir: StartDir,
    width: usize,
    height: usize,
) -> u32 {
    // let mut steps = 0;
    let mut steps_since_last_new_cell = 0;
    let mut visited = HashSet::new();
    let mut guard_idx = start_idx;
    let mut guard_dir = start_dir;
    let mut guard_action = get_guard_action(guard_idx, guard_dir, cells, width, height);
    while guard_action != GuardAction::Exit && steps_since_last_new_cell < MAX_STEPS {
        // steps += 1;
        match guard_action {
            GuardAction::Exit => {}
            GuardAction::Turn => match guard_dir {
                StartDir::Up => {
                    guard_dir = StartDir::Right;
                }
                StartDir::Right => {
                    guard_dir = StartDir::Down;
                }
                StartDir::Down => {
                    guard_dir = StartDir::Left;
                }
                StartDir::Left => {
                    guard_dir = StartDir::Up;
                }
                StartDir::None => {}
            },
            GuardAction::Advance => match guard_dir {
                StartDir::Up => {
                    guard_idx = guard_idx - width;
                }
                StartDir::Right => {
                    guard_idx = guard_idx + 1;
                }
                StartDir::Down => {
                    guard_idx = guard_idx + width;
                }
                StartDir::Left => {
                    guard_idx = guard_idx - 1;
                }
                StartDir::None => {}
            },
        }
        let current_cell_visits = visited.len();
        visited.insert(guard_idx);
        if current_cell_visits == visited.len() {
            steps_since_last_new_cell += 1
        } else {
            steps_since_last_new_cell = 0
        }
        guard_action = get_guard_action(guard_idx, guard_dir, cells, width, height);
    }
    if steps_since_last_new_cell == MAX_STEPS {
        0
    } else {
        visited.len() as u32
    }
}

#[derive(PartialEq, Eq)]
pub enum GuardAction {
    Advance,
    Exit,
    Turn,
}

pub fn get_guard_action(
    guard_idx: usize,
    guard_dir: StartDir,
    cells: &Vec<u8>,
    width: usize,
    height: usize,
) -> GuardAction {
    match guard_dir {
        StartDir::Up => {
            // check cell directly above
            if guard_idx < width {
                GuardAction::Exit
            } else {
                let next_cell = cells[guard_idx - width];
                match next_cell {
                    0 => GuardAction::Advance,
                    1 => GuardAction::Turn,
                    _ => GuardAction::Advance,
                }
            }
        }
        StartDir::Left => {
            if guard_idx % width == 0 {
                GuardAction::Exit
            } else {
                let next_cell = cells[guard_idx - 1];
                match next_cell {
                    0 => GuardAction::Advance,
                    1 => GuardAction::Turn,
                    _ => GuardAction::Advance,
                }
            }
        }
        StartDir::Down => {
            if guard_idx > ((height - 1) * width) {
                GuardAction::Exit
            } else {
                let next_cell = cells[guard_idx + width];
                match next_cell {
                    0 => GuardAction::Advance,
                    1 => GuardAction::Turn,
                    _ => GuardAction::Advance,
                }
            }
        }
        StartDir::Right => {
            if guard_idx % width == (width - 1) {
                GuardAction::Exit
            } else {
                let next_cell = cells[guard_idx + 1];
                match next_cell {
                    0 => GuardAction::Advance,
                    1 => GuardAction::Turn,
                    _ => GuardAction::Advance,
                }
            }
        }
        StartDir::None => GuardAction::Advance,
    }
}
