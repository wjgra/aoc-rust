use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

macro_rules! noop { () => (); }

// Get a grid of where fallen bytes land
fn get_input(input_path: &str, max_idx: usize, fallen_bytes: usize) -> Vec<Vec<bool>> {
    // println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);
    let coords_re = Regex::new(r"([0-9]*),([0-9]*)").unwrap();

    let mut map_data = vec![vec![false; max_idx + 1]; max_idx + 1];
    for line in buf.lines().map(|l| l.unwrap()).take(fallen_bytes) {
        for (_, [x, y]) in coords_re.captures_iter(&line).map(|l| l.extract()) {
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            map_data[y][x] = true;
        }
    }
    return map_data;
}

fn draw_map(map_data: &Vec<Vec<bool>>) {
    for row in map_data.iter() {
        for col in row.iter() {
            if *col {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

fn get_distance(map_data: &Vec<Vec<bool>>, max_idx: usize) -> Option<i32> {
    let mut to_process = PriorityQueue::new();
    to_process.push((0, 0), Reverse(0));

    let mut visited = vec![vec![false; max_idx + 1]; max_idx + 1];

    while !to_process.is_empty() {
        let (current_pos, Reverse(current_distance)) = to_process.pop().unwrap();
        if visited[current_pos.0 as usize][current_pos.1 as usize] {
            unreachable!();
        }

        // Check if goal reached
        if current_pos == (max_idx as i32, max_idx as i32) {
            return Some(current_distance);
            // println!("Part one: {current_distance}");
            // break;
        }

        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
            if (next_pos.0 < 0) || (next_pos.1 < 0) || (next_pos.0 > (max_idx as i32)) || (next_pos.1 > (max_idx as i32)) {
                // println!("{:?} out of range", next_pos);
                continue;
            }
            if !visited[next_pos.0 as usize][next_pos.1 as usize] && !map_data[next_pos.0 as usize][next_pos.1 as usize] {
                // println!("Adding {:?}", next_pos);
                to_process.push(next_pos, Reverse(current_distance + 1));
            }
        }

        // Mark as visited
        visited[current_pos.0 as usize][current_pos.1 as usize] = true;
    }
    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_18_example.txt"} else { &args[1] };

    let max_idx = if args.len() < 2 { 6 } else { 70 };
    let fallen_bytes = if args.len() < 2 { 12 } else { 1024 };

    // Part one - find shortest path from (0,0) to (max_idx,max_idx)
    let map_data = get_input(&input_path, max_idx, fallen_bytes);
    println!("After {fallen_bytes} bytes have fallen");
    draw_map(&map_data);

    match get_distance(&map_data, max_idx) {
        Some(distance) => println!("Part one: {distance}"),
        None => unreachable!()
    }

    // Part two - find first byte for which there is no escape
    let mut fallen_bytes = fallen_bytes;
    loop {
        println!("Trying {fallen_bytes}");
        let map_data = get_input(&input_path, max_idx, fallen_bytes);
        match get_distance(&map_data, max_idx) {
            Some(distance) => {noop!();},
            None => {println!("Part two idx: {fallen_bytes}"); break; }
        }
        fallen_bytes += 1;
    }
}