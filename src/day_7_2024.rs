use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input(input_path: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);
    let mut targets = vec![];
    let mut operands = vec![];

    for line in buf.lines().map(|l| l.unwrap()) {
        let split_line: Vec<&str> = line.trim().split(':').collect();
        targets.push(split_line[0].parse::<i64>().unwrap());
        let line_operands: Vec<i64> = split_line[1].split_whitespace()
            .map(|n| n.parse::<i64>().unwrap()).collect();
        operands.push(line_operands);
    }

    return (targets, operands);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_7_example.txt"} else { &args[1] };

    let (target_list, operands_list) = get_input(input_path);

    // Part one - sum of targets achievable with * and +
    let mut total: i64 = 0;
    for (target, operands) in target_list.iter().zip(operands_list.iter()) {
        let mut reachable: Vec<(i64, i64)> = vec![(operands[0], 0)]; // Numbers reachable by a given index

        while reachable.is_empty() == false {
            let current = reachable.pop().unwrap();
            let is_last = current.1 == (operands.len() - 1) as i64;
            if current.0 == *target && is_last {
                // Target is achievable
                total += *target as i64;
                break;
            }
            else if !is_last {
                let next_idx = current.1 + 1;
                reachable.push((current.0 + operands[next_idx as usize], next_idx));
                reachable.push((current.0 * operands[next_idx as usize], next_idx));
            }
        }        
    }
    println!("Part one: {total}");

    // Part one - sum of targets achievable with *, + and concat
    let mut total: i64 = 0;
    for (target, operands) in target_list.iter().zip(operands_list.iter()) {
        let mut reachable: Vec<(i64, i64)> = vec![(operands[0], 0)]; // Numbers reachable by a given index

        while reachable.is_empty() == false {
            let current = reachable.pop().unwrap();
            let is_last = current.1 == (operands.len() - 1) as i64;
            if current.0 == *target && is_last {
                // Target is achievable
                total += *target as i64;
                break;
            }
            else if !is_last {
                let next_idx = current.1 + 1;
                reachable.push((current.0 + operands[next_idx as usize], next_idx));
                reachable.push((current.0 * operands[next_idx as usize], next_idx));
                
                let operand_digits = operands[next_idx as usize].to_string().chars().count();
                let base: i64 = 10;
                reachable.push((current.0 * base.pow(operand_digits as u32) + operands[next_idx as usize], next_idx))
            }
        }        
    }
    println!("Part two: {total}");
}