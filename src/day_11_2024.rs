use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn get_input(input_path: &str) -> Vec<u64> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    let input: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    let input = input[0].trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    return input;
}

#[allow(dead_code)]
// This is my old implementation - superceded by the histogram version!
fn get_stones_after_blinks(input_stones: &Vec<u64>, num_blinks: i32) -> u64 {
    let mut stones_to_process = vec![];
    for &stone in input_stones {
        stones_to_process.push((stone, 0)); // The first number is the value of the stone, the second is the number of blinks that have taken place
    }

    
    let mut total_stones: u64 = 0;
    while !stones_to_process.is_empty() {
        let (stone_val, current_blinks) = stones_to_process.pop().unwrap();
        // If enough blinks have happened, add to 
        if current_blinks == num_blinks {
            total_stones += 1;
        }
        else {
            if stone_val == 0 {
                stones_to_process.push((1, current_blinks + 1));
            }
            else {
                let num_digits = stone_val.ilog10() + 1;
                // Number of digits is even
                if num_digits % 2 == 0 {
                    let divisor = (10 as u64).pow(num_digits / 2);
                    stones_to_process.push((stone_val / divisor, current_blinks + 1));
                    stones_to_process.push((stone_val % divisor, current_blinks + 1));
                }
                else {
                    stones_to_process.push((2024 * stone_val, current_blinks + 1));
                }
            }            
        }
    }

    return total_stones;
}

fn insert_or_add_stone_num(stones: &mut HashMap<u64, u64>, stone_val: u64, num_to_add: u64) {
    match stones.get_mut(&stone_val) {
        Some(num) => *num += num_to_add,
        None => { stones.insert(stone_val, num_to_add); }
    }
}

fn get_stones_after_blinks_histogram(input_stones: &Vec<u64>, num_blinks: i32) -> u64 {
    let mut stones_to_process = HashMap::new();
    for &stone in input_stones {
        stones_to_process.insert(stone, 1 as u64); // The first number is the value of the stone, the second is the count
    }

    for _ in 0..num_blinks {
        let mut next_stones_to_process = HashMap::new();
        for (stone_val, num_stones) in stones_to_process {
            if stone_val == 0 {
                insert_or_add_stone_num(&mut next_stones_to_process, 1, num_stones);
            }
            else {
                let num_digits = stone_val.ilog10() + 1;
                if num_digits % 2 == 0 {
                    let divisor = (10 as u64).pow(num_digits / 2);
                    insert_or_add_stone_num(&mut next_stones_to_process, stone_val / divisor, num_stones);
                    insert_or_add_stone_num(&mut next_stones_to_process, stone_val % divisor, num_stones);
                }
                else {
                    insert_or_add_stone_num(&mut next_stones_to_process, 2024 * stone_val, num_stones);
                }
            }
        }
        stones_to_process = next_stones_to_process;
    }

    let mut total_stones = 0;
    for (_, num) in stones_to_process {
        total_stones += num;
    }
    return total_stones;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_11_example.txt" } else { &args[1] };

    let input = get_input(&input_path);

    // Part one - number of stones after 25 blinks
    let num_stones = get_stones_after_blinks_histogram(&input, 25);
    println!("Part one: {num_stones}");

    // Part two - number of stones after 75 blinks
    let num_stones = get_stones_after_blinks_histogram(&input, 75);
    println!("Part two: {num_stones}");
}