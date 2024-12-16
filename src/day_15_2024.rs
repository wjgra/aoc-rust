use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

macro_rules! noop { () => (); }

fn get_input(input_path: &str) -> (Vec<Vec<char>>, (i32, i32), Vec<char>) {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    let mut output = vec![];
    let mut robot_pos = (0,0);
    let mut directions = vec![];

    for line in buf.lines().map(|l| l.unwrap()) {
        if line.len() != 0 {
            let first_char = line.chars().next().unwrap();
            match first_char {
                '#' => {
                    let mut map_line: Vec<char> = line.chars().collect();
                    // let robot_char = '@';
                    match map_line.iter().position(|n| *n == '@') {
                        Some(pos) => {
                            map_line[pos] = '.'; // Remove robot from array
                            robot_pos = (output.len() as i32, pos as i32);
                        },
                        _ => { noop!(); }
                    }
                    output.push(map_line);
                },
                'v' | '^' | '<' | '>' => {
                    let mut new_dirs = line.chars().collect();
                    directions.append(&mut new_dirs);
                },
                _ => unreachable!()
            }
        } 
    }

    println!("Robot at {}, {}", robot_pos.0, robot_pos.1);
    return (output, robot_pos, directions);
}

fn print_state(map: &Vec<Vec<char>>) {
    for line in map.iter() {
        for col in line.iter() {
            print!("{col}");
        }
        println!("");
    }
}

fn get_dir_vec(input: &char) -> (i32, i32) {
    match input {
        '^' => return (-1, 0),
        'v' => return (1, 0),
        '<' => return (0, -1),
        '>' => return (0, 1),
        _ => unreachable!()
    }
}

fn get_score(map: &Vec<Vec<char>>) -> i64 {
    let mut total = 0 as i64;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' || map[i][j] == '[' {
                total += (100 * i + j) as i64;
            }
        }
    }
    return total;
}

fn get_double_width_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![];
    for line in map {
        let mut new_line: Vec<char> = vec![];
        for char in line {
            if *char == 'O' {
                new_line.push('[');
                new_line.push(']');
            }
            else {
                new_line.push(*char);
                new_line.push(*char);
            }
        }
        output.push(new_line);
    }
    return output;
}

// Box is identified by coords of left bracket '['
/* 
[][] or [] or [][]
 []     []
*/
fn can_boxes_move(box_pos: (i32, i32), dir: (i32, i32), current_state: &mut Vec<Vec<char>>) -> bool {
    println!("Checking box at {}, {}", box_pos.0, box_pos.1);
    // Check if box is moved into wall
    let lhs_next_pos = (box_pos.0 + dir.0, box_pos.1 + dir.1);
    let rhs_next_pos = (box_pos.0 + dir.0, box_pos.1 + dir.1 + 1);
    let lhs_next = current_state[lhs_next_pos.0 as usize][lhs_next_pos.1 as usize];
    let rhs_next = current_state[rhs_next_pos.0 as usize][rhs_next_pos.1 as usize];
    if lhs_next == '#' || rhs_next == '#' {
        return false;
    }
    if (lhs_next == '.' && rhs_next == '.') || (lhs_next == '.' && rhs_next_pos == box_pos) || (rhs_next == '.' && lhs_next_pos == (box_pos.0, box_pos.1 + 1)) {
        return true;
    }

    // Check directly vertical box push
    if dir.1 == 0 && lhs_next == '[' && rhs_next == ']' {
        return can_boxes_move(lhs_next_pos, dir, current_state);
    }

    // Check horizontal box push
    if dir.0 == 0 {
        match dir.1 {
            1 => {return can_boxes_move((box_pos.0, box_pos.1 + 2), dir, current_state)}, // right push
            -1 => {return can_boxes_move((box_pos.0, box_pos.1 - 2), dir, current_state)}, // left push
            _ => unreachable!()
        }
    }

    // Check vertical offset box push
    if dir.1 == 0 {
        let mut can_box_move = true;
        if lhs_next == ']' {
            can_box_move &= can_boxes_move((lhs_next_pos.0, lhs_next_pos.1 - 1), dir, current_state);
        }
        if rhs_next == '['{ 
            can_box_move &= can_boxes_move(rhs_next_pos, dir, current_state);
        }
        return can_box_move;
    }

    unreachable!();
}

fn move_boxes(box_pos: (i32, i32), dir: (i32, i32), current_state: &mut Vec<Vec<char>>) {
    println!("Moving box at {}, {}", box_pos.0, box_pos.1);
    let lhs_next_pos = (box_pos.0 + dir.0, box_pos.1 + dir.1);
    let rhs_next_pos = (box_pos.0 + dir.0, box_pos.1 + dir.1 + 1);
    let lhs_next = current_state[lhs_next_pos.0 as usize][lhs_next_pos.1 as usize];
    let rhs_next = current_state[rhs_next_pos.0 as usize][rhs_next_pos.1 as usize];

    if lhs_next == '#' || rhs_next == '#' {
        unreachable!();
    }

    // Move downstream boxes first
    if !((lhs_next == '.' && rhs_next == '.') || (lhs_next == '.' && rhs_next_pos == box_pos) || (rhs_next == '.' && lhs_next_pos == (box_pos.0, box_pos.1 + 1))) {
        // Check directly vertical box push
        if dir.1 == 0 && lhs_next == '[' && rhs_next == ']' {
            move_boxes(lhs_next_pos, dir, current_state);
        }

        // Check horizontal box push
        else if dir.0 == 0 {
            match dir.1 {
                1 => {move_boxes((box_pos.0, box_pos.1 + 2), dir, current_state)}, // right push
                -1 => {move_boxes((box_pos.0, box_pos.1 - 2), dir, current_state)}, // left push
                _ => unreachable!()
            }
        }

        // Check vertical offset box push
        else if dir.1 == 0 {
            if lhs_next == ']' {
                move_boxes((lhs_next_pos.0, lhs_next_pos.1 - 1), dir, current_state);
            }
            if rhs_next == '['{ 
                move_boxes(rhs_next_pos, dir, current_state);
            }
        }
    }

    current_state[box_pos.0 as usize][box_pos.1 as usize] = '.';
    current_state[box_pos.0 as usize][(box_pos.1 + 1) as usize] = '.';
    current_state[lhs_next_pos.0 as usize][lhs_next_pos.1 as usize] = '[';
    current_state[rhs_next_pos.0 as usize][rhs_next_pos.1 as usize] = ']';
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_15_example.txt"} else { &args[1] };

    let (initial_grid, robot_initial_pos, directions) = get_input(&input_path);

    println!("State before:");
    print_state(&initial_grid);

    let height = initial_grid.len() as i32;
    let width = initial_grid[0].len() as i32;

    // Part one - move robot and boxes according to direction list
    let mut current_robot_pos = robot_initial_pos;
    let mut current_state = initial_grid.clone();
    for dir in &directions {
        let dir = get_dir_vec(dir);
        let next_robot_pos = (current_robot_pos.0 + dir.0, current_robot_pos.1 + dir.1);
        if next_robot_pos.0 < 0 || next_robot_pos.0 >= height || next_robot_pos.1 < 0 || next_robot_pos.1 >= width {
            continue; // Next pos would not be inside the grid
        }
        match current_state[next_robot_pos.0 as usize][next_robot_pos.1 as usize] {
            '#' => continue, // No space in front of robot
            '.' => {
                // Space for robot to move
                current_robot_pos = next_robot_pos
            },
            'O' => {
                // Box in front - check whether boxes can move
                let mut scan_pos = next_robot_pos;
                loop {
                    scan_pos = (scan_pos.0 + dir.0, scan_pos.1 + dir.1);
                    match current_state[scan_pos.0 as usize][scan_pos.1 as usize] {
                        '.' => {
                            // Found a gap - move boxes into gap and move robot
                            current_state[scan_pos.0 as usize][scan_pos.1 as usize] = 'O';
                            current_state[next_robot_pos.0 as usize][next_robot_pos.1 as usize] = '.';
                            current_robot_pos = next_robot_pos;
                            break;
                        },
                        '#' => {
                            // Reached the edge - no movement possible
                            break;
                        },
                        'O' => {
                            // Another box - keep scanning
                            continue;
                        }
                        _ => unreachable!()
                    }
                }
            },
            _ => unreachable!()
        }
    }
    println!("State after:");
    print_state(&current_state);
    let score = get_score(&current_state);
    println!("Part one: {score}");

    // Part two - everything but the robot is twice as wide!
    let initial_state = get_double_width_map(&initial_grid);
    let height = initial_state.len() as i32;
    let width= initial_state[0].len() as i32;
    println!("State before:");
    print_state(&initial_state);
    current_state = initial_state.clone();

    let robot_initial_pos = (robot_initial_pos.0, robot_initial_pos.1 * 2);
    let mut current_robot_pos = robot_initial_pos;
    for dir in &directions {
        let dir = get_dir_vec(dir);
        println!("Dir: {:?}", dir);
        let next_robot_pos = (current_robot_pos.0 + dir.0, current_robot_pos.1 + dir.1);
        if next_robot_pos.0 < 0 || next_robot_pos.0 >= height || next_robot_pos.1 < 0 || next_robot_pos.1 >= width {
            continue; // Next pos would not be inside the grid
        }
        match current_state[next_robot_pos.0 as usize][next_robot_pos.1 as usize] {
            '#' => continue, // No space in front of robot
            '.' => {
                // Space for robot to move
                current_robot_pos = next_robot_pos
            },
            '[' => {
                // Box in front - check whether boxes can move
                // Recursive solution? Box can only move if boxes in front of it (three possible positions) can also move
                if can_boxes_move(next_robot_pos, dir, &mut current_state) {
                    move_boxes(next_robot_pos, dir, &mut current_state);
                    current_robot_pos = next_robot_pos;
                }
                else {
                    println!("Can't move!");
                }

            },
            ']' => {
                // Box in front - robot pushing RHS of box
                let box_pos = (next_robot_pos.0, next_robot_pos.1 - 1);
                if can_boxes_move(box_pos, dir, &mut current_state) {
                    move_boxes(box_pos, dir, &mut current_state);
                    current_robot_pos = next_robot_pos;
                }
                else {
                    println!("Can't move!");
                }
            },
            _ => unreachable!()
        }
        // break;
        // print_state(&current_state);
    }
    println!("State after:");
    print_state(&current_state);
    let score = get_score(&current_state);
    println!("Part two: {score}");

}
