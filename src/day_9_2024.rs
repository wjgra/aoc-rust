use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input(input_path: &str) -> Vec<i32> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let input: Vec<String> = BufReader::new(input).lines()
        .map(|l| l.unwrap())
        .collect();

    return input[0].chars().map(|n| n.to_digit(10).unwrap() as i32).collect()
}

fn add_block_to_checksum(checksum: &mut usize, checksum_idx: &mut usize, block_id_number: usize, block_size: usize) {
    for _ in 0..block_size {
        *checksum += *checksum_idx * block_id_number;
        *checksum_idx += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_9_example.txt"} else { &args[1] };
    let input = get_input(input_path);

    // Part one - move blocks from right to the left
    let mut working_disk_info = input.clone();

    let mut checksum = 0;
    let mut checksum_idx = 0;

    let mut left = 0;
    let mut right = input.len() - 1;

    // Even indices are data, odd are spaces
    while left <= right {
        match left % 2 {
            // There is data in leftmost block - add to checksum
            0 => {
                add_block_to_checksum(&mut checksum, &mut checksum_idx, left / 2, working_disk_info[left] as usize);
                left += 1; // Move to next block
            },
            // There is no data in leftmost block - fill from right
            1 => {
                match right % 2 {
                    0 => {
                        let id_number = right / 2;
                        // Rightmost data block completely fills leftmost
                        if working_disk_info[right] >= working_disk_info[left] {
                            add_block_to_checksum(&mut checksum, &mut checksum_idx, id_number, working_disk_info[left] as usize);

                            if working_disk_info[right] > working_disk_info[left] {
                                working_disk_info[right] -= working_disk_info[left];
                            }
                            else {
                                // We are done with rightmost block
                                right -= 1;
                            }

                            left += 1;
                        }
                        // Rightmost data block does not completely fill leftmost
                        else {
                            add_block_to_checksum(&mut checksum, &mut checksum_idx, id_number, working_disk_info[right] as usize);

                            // We are not done with leftmost block
                            working_disk_info[left] -= working_disk_info[right];

                            right -= 1;
                        }
                    },
                    // There is no data in the rightmost block - skip it!
                    1 => {
                        right -= 1;
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    println!("Part one: {checksum}");

    // Part one - move blocks from right to the left
    let mut working_disk_info = input.clone();

    let mut checksum = 0;
    let mut checksum_idx = 0;

    let mut left = 0;
    let mut right = input.len() - 1;

    // Even indices are data, odd are spaces
    while left <= right {
        match left % 2 {
            // There is data in leftmost block - add to checksum
            0 => {
                add_block_to_checksum(&mut checksum, &mut checksum_idx, left / 2, working_disk_info[left] as usize);
                left += 1; // Move to next block
            },
            // There is no data in leftmost block - fill from right
            1 => {
                match right % 2 {
                    0 => {
                        let id_number = right / 2;

                        // Try each gap from left until it fits - if it doesn't fit in any then just leave in place and calculate checksum
                        let mut try_left = left;
                        let mut try_checksum_idx = checksum_idx;
                        while try_left < right {
                            // Try to fit data into next free block
                            if working_disk_info[try_left] >= working_disk_info[right] {
                                // There is enough space to fit the right block in the left block, but left block may already be partially used
                                try_checksum_idx += (input[try_left] - working_disk_info[try_left]) as usize;

                                // Calculate  checksum
                                add_block_to_checksum(&mut checksum, &mut try_checksum_idx, id_number, working_disk_info[right] as usize);

                                if (working_disk_info[try_left] == working_disk_info[right]) && (try_left == left) {
                                    // We're done with leftmost block
                                    checksum_idx += input[left] as usize;
                                    left += 1;
                                }
                                else {
                                    working_disk_info[try_left] -= input[right];
                                }
                                right -= 1;
                                break;
                            }
                            else {
                                // Move on to next gap
                                try_checksum_idx += (input[try_left] + input[try_left + 1]) as usize; // This is the start of the next free block
                                try_left += 2; // Next block to try is two along
                            }
                        }

                        // If block didn't fit anywhere, calculate checksum in place
                        if try_left >= right {
                            // try_checksum_idx currently points to start of gap after rightmost
                            try_checksum_idx -= working_disk_info[right] as usize;
                            add_block_to_checksum(&mut checksum, &mut try_checksum_idx, id_number, working_disk_info[right] as usize);
                            right -= 1;
                        }           
                    },
                    // There is no data in the rightmost block - skip it!
                    1 => {
                        right -= 1;
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    println!("Part two: {checksum}");
}