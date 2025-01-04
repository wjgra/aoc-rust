use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

macro_rules! noop { () => (); }

enum ReindeerDir {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

// Relates dir enum to index vectors
fn dir_to_vec(dir: &ReindeerDir) -> (i32, i32) {
    match dir {
        ReindeerDir::NORTH => return (-1, 0),
        ReindeerDir::EAST => return (0, 1),
        ReindeerDir::SOUTH => return (1, 0),
        ReindeerDir::WEST => return (0, -1)
    }
}

fn dir_to_idx(dir: &ReindeerDir) -> usize {
    match dir {
        ReindeerDir::NORTH => return 0,
        ReindeerDir::EAST => return 1,
        ReindeerDir::SOUTH => return 2,
        ReindeerDir::WEST => return 3
    }
}

fn idx_to_dir(idx: usize) -> ReindeerDir {
    match idx {
        0 => return ReindeerDir::NORTH,
        1 => return ReindeerDir::EAST,
        2 => return ReindeerDir::SOUTH,
        3 => return ReindeerDir::WEST,
        _ => unreachable!()
    }
}

fn get_input(input_path: &str) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    let mut output = vec![];
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    for line in buf.lines().map(|l| l.unwrap()) {
        let mut map_line: Vec<char> = line.chars().collect();
        // Check for start point
        match map_line.iter().position(|n| *n == 'S') {
            Some(pos) => {
                map_line[pos] = '.'; // Remove start from array
                start_pos = (output.len() as i32, pos as i32);
                println!("Start at {:?}", start_pos);
            },
            _ => { noop!(); }
        }
        // Check for end point
        match map_line.iter().position(|n| *n == 'E') {
            Some(pos) => {
                map_line[pos] = '.'; // Remove end from array
                end_pos = (output.len() as i32, pos as i32);
                println!("End at {:?}", end_pos);
            },
            _ => { noop!(); }
        }
        output.push(map_line);
    }

    return (output, start_pos, end_pos);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_16_example.txt"} else { &args[1] };

    let (map, start_pos, end_pos) = get_input(&input_path);

    // Part one - find cheapest path through maze
    let mut visited = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
    let mut scores = vec![vec![vec![0; 4]; map[0].len()]; map.len()];
    let mut on_best_path = vec![vec![vec![false; 4]; map[0].len()]; map.len()];

    let mut to_process = PriorityQueue::new();

    to_process.push((start_pos, dir_to_idx(&ReindeerDir::EAST)), Reverse(0));

    while !to_process.is_empty() {     
        let (current_pos, Reverse(current_score)) = to_process.pop().unwrap();
        if visited[(current_pos.0).0 as usize][(current_pos.0).1 as usize][current_pos.1] {
            unreachable!(); // ??
        }

        // Check if an endpoint has been reached - no need to check for all as the first will be fastest by priority queue
        // if current_pos.0 == end_pos {
        //     println!("Part one score: {current_score}");
        //     // break;
        // }

        // Add step forward, if not already visited and no wall in the way
        let forward_dir = dir_to_vec(&idx_to_dir(current_pos.1));
        let next_pos = (((current_pos.0).0 + forward_dir.0), ((current_pos.0).1 + forward_dir.1));
        if map[ next_pos.0 as usize][next_pos.1 as usize] == '.' && !visited[next_pos.0 as usize][next_pos.1 as usize][current_pos.1] {
            to_process.push((next_pos, current_pos.1), Reverse(current_score + 1));
        }

        // Add two turns, if not already visited
        let left_turn_idx = (((current_pos.1 as i32) + 3) % 4) as usize;
        if !visited[(current_pos.0).0 as usize][(current_pos.0).1 as usize][left_turn_idx] {
            to_process.push((current_pos.0, left_turn_idx), Reverse(current_score + 1000));
        }

        let right_turn_idx = (((current_pos.1 as i32) + 1) % 4) as usize;
        if !visited[(current_pos.0).0 as usize][(current_pos.0).1 as usize][right_turn_idx] {
            to_process.push((current_pos.0, right_turn_idx), Reverse(current_score + 1000));
        }

        // Mark as visited
        visited[(current_pos.0).0 as usize][(current_pos.0).1 as usize][current_pos.1] = true;

        // Set score
        scores[(current_pos.0).0 as usize][(current_pos.0).1 as usize][current_pos.1] = current_score;
    }
    let &best_score = scores[end_pos.0 as usize][end_pos.1 as usize].iter().min().unwrap();
    println!("Part one: {best_score}");

    // Backtrack to find points on best path
    let mut to_process = vec![];
    
    for idx in 0..4 {
        if scores[end_pos.0 as usize][end_pos.1 as usize][idx] == best_score {
            to_process.push((end_pos, idx));
        }
    }

    while !to_process.is_empty() {
        let (current_pos, current_dir) = to_process.pop().unwrap();
        let current_score = scores[current_pos.0 as usize][current_pos.1 as usize][current_dir];

        // Check if step back is on best path
        let dir_vec = dir_to_vec(&idx_to_dir(current_dir));
        let last_pos = (current_pos.0 - dir_vec.0, current_pos.1 - dir_vec.1);
        if scores[last_pos.0 as usize][last_pos.1 as usize][current_dir] == (current_score - 1){
            to_process.push((last_pos, current_dir));
        }

        // Check if each reverse turn is on a best path
        let left_turn_idx = (((current_dir as i32) + 3) % 4) as usize;
        if scores[current_pos.0 as usize][current_pos.1 as usize][left_turn_idx] == (current_score - 1000) {
            to_process.push((current_pos, left_turn_idx));
        }

        let right_turn_idx = (((current_dir as i32) + 1) % 4) as usize;
        if scores[current_pos.0 as usize][current_pos.1 as usize][right_turn_idx] == (current_score - 1000) {
            to_process.push((current_pos, right_turn_idx));
        }

        on_best_path[current_pos.0 as usize][current_pos.1 as usize][current_dir] = true;
    }
    let mut num_on_path = 0;
    for row in on_best_path.iter() {
        for pos in row.iter() {
            for &dir in pos {
                if dir {
                    num_on_path += 1;
                    break;
                }
            }
        }
    }
    println!("Part two: {num_on_path}");

}
