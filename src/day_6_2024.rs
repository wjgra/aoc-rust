use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input(input_path: &str) -> Vec<Vec<char>> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);
    let mut output = vec![];
    for line in buf.lines().map(|l| l.unwrap()) {
        let line_chars: Vec<char> = line.chars().collect();
        output.push(line_chars);
    }
    return output;
}

enum GuardDir {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

// Relates dir enum to index vectors
fn guard_dir_to_vec(dir: &GuardDir) -> (i32, i32) {
    match dir {
        GuardDir::UP => return (-1, 0),
        GuardDir::RIGHT => return (0, 1),
        GuardDir::DOWN => return (1, 0),
        GuardDir::LEFT => return (0, -1)
    }
}

// The guard turns right at each stage - get the next direction
fn next_guard_dir(dir: &GuardDir) -> GuardDir {
    match dir {
        GuardDir::UP => return GuardDir::RIGHT,
        GuardDir::RIGHT => return GuardDir::DOWN,
        GuardDir::DOWN => return GuardDir::LEFT,
        GuardDir::LEFT => return GuardDir::UP
    }
}

fn guard_dir_to_char(dir: &GuardDir) -> char {
    match dir {
        GuardDir::UP => return '^',
        GuardDir::RIGHT => return '>',
        GuardDir::DOWN => return 'v',
        GuardDir::LEFT => return '<'
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 { "input/day_6_example.txt"} else { &args[1] };

    let mut map = get_input(&input_path);

    // Part one - how many positions does the guard occupy?

    // Find the intial position of the guard (who is pointing up)
    let mut guard_dir = GuardDir::UP;
    let mut guard_i: i32 = -1;
    let mut guard_j: i32 = -1;

    let dim_i = map.len();
    let dim_j = map[0].len();

    for i in 0..dim_i {
        for j in 0..dim_j {
            if map[i][j] == '^' {
                guard_i = i as i32;
                guard_j = j as i32;
                break;
            }
        }
    }

    if guard_i == -1 || guard_j == -1 {
        panic!("guard not found");
    }
    else {
        println!("Guard found at {guard_i}, {guard_j}");
    }

    let mut guard_squares = 1; // Squares covered by guard

    loop {
        let dir_vec = guard_dir_to_vec(&guard_dir);
        let next_square = (dir_vec.0 + guard_i, dir_vec.1 + guard_j);

        // If next square is outside map, end
        if (next_square.0 == -1) || (next_square.0 == dim_i as i32) || (next_square.1 == -1) || (next_square.1 == dim_j as i32) {
            break;
        }
        // If next square is obstacle, change dir
        else if map[next_square.0 as usize][next_square.1 as usize] == '#' {
            guard_dir = next_guard_dir(&guard_dir);
        }
        // Else, move to next square and increase count if not already visited
        else {
            if map[next_square.0 as usize][next_square.1 as usize] != '^' {
                guard_squares += 1;
            }
            
            map[next_square.0 as usize][next_square.1 as usize] = '^'; // Mark as visited with guard symbol           
            
            guard_i = next_square.0;
            guard_j = next_square.1;
        }
    }

    println!("Guard squares: {guard_squares}");

    let map = get_input(&input_path);

    // Part two - how many places can we put an obstacle that cause a loop?

    /* 
     * REMARK: the solution to part two is pretty terrible, and takes about 10s to run on my PC. Two ways to
     * speed it up: a) only check for obstacles on the path from part one and b) only check for matching dirs
     * at the point at which you change directions. I will implement both when I get the chance.
     */

    // Find the intial position of the guard (who is pointing up)
    let mut guard_i: i32 = -1;
    let mut guard_j: i32 = -1;

    let dim_i = map.len();
    let dim_j = map[0].len();

    for i in 0..dim_i {
        for j in 0..dim_j {
            if map[i][j] == '^' {
                guard_i = i as i32;
                guard_j = j as i32;
                break;
            }
        }
    }

    if guard_i == -1 || guard_j == -1 {
        panic!("guard not found");
    }
    else {
        println!("Guard found at {guard_i}, {guard_j}");
    }

    let mut obstacle_squares = 0;
    // Test each square to see if it works as an obstacle
    for obs_i in 0..dim_i {
        for obs_j in 0..dim_j {
            // Test each square by walking the guard until either exit or arriving in the same square in the same direction
            let mut map_copy = map.clone();
            let mut guard_i_copy = guard_i.clone();
            let mut guard_j_copy = guard_j.clone();
            let mut guard_dir = GuardDir::UP;
            
            if map_copy[obs_i][obs_j] == '.' {
                map_copy[obs_i][obs_j] = '#';
            }
            else {
                continue;
            }
            
            let exited;
            loop {
                let dir_vec = guard_dir_to_vec(&guard_dir);
                let next_square = (dir_vec.0 + guard_i_copy, dir_vec.1 + guard_j_copy);
        
                // If next square is outside map, end
                if (next_square.0 == -1) || (next_square.0 == dim_i as i32) || (next_square.1 == -1) || (next_square.1 == dim_j as i32) {
                    exited = true;
                    break;
                }
                // If next square is obstacle, change dir
                else if map_copy[next_square.0 as usize][next_square.1 as usize] == '#' {
                    guard_dir = next_guard_dir(&guard_dir);
                }
                // Else, move to next square and check if already visited in same direction
                else {
                    let current_dir_char = guard_dir_to_char(&guard_dir);
                    if map_copy[next_square.0 as usize][next_square.1 as usize] == current_dir_char {
                        exited = false;
                        break;
                    }
                    
                    map_copy[next_square.0 as usize][next_square.1 as usize] = current_dir_char; // Mark as visited with appropriate direction       
                    
                    guard_i_copy = next_square.0;
                    guard_j_copy = next_square.1;
                }
            }

            if exited == false {
                obstacle_squares += 1;

            }
        }
    }

    println!("Obstacle squares: {obstacle_squares}");
}

