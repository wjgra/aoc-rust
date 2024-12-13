use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Clone)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    objective: (i64, i64)
}

fn get_input(input_path: &str) -> Vec<Machine> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    let mut line_num = 0;
    let mut current_machine = Machine {
        button_a: (0,0),
        button_b: (0,0),
        objective: (0,0)
    };
    let mut output = vec![];

    let button_a_re = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let button_b_re = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    for line in buf.lines().map(|l| l.unwrap()) {
        match line_num % 4 {
            0 => {
                for (_, [a_x, a_y]) in button_a_re.captures_iter(&line)
                    .map(|n| n.extract()) {
                        current_machine.button_a = (a_x.parse().unwrap(), a_y.parse().unwrap());
                    }
            },
            1 => {
                for (_, [b_x, b_y]) in button_b_re.captures_iter(&line)
                .map(|n| n.extract()) {
                    current_machine.button_b = (b_x.parse().unwrap(), b_y.parse().unwrap());
                }
            },
            2 => {
                for (_, [p_x, p_y]) in prize_re.captures_iter(&line)
                .map(|n| n.extract()) {
                    current_machine.objective = (p_x.parse().unwrap(), p_y.parse().unwrap());
                }
                println!("Read machine a: {}, {}, b: {}, {}, obj: {}, {}",
                current_machine.button_a.0,
                current_machine.button_a.1,
                current_machine.button_b.0,
                current_machine.button_a.1,
                current_machine.objective.0,
                current_machine.objective.1);
            output.push(current_machine.clone());
            },
            3 => {
            },
            _ => unreachable!()
        }
        line_num += 1;
    }

    return output;
}

fn get_token_count(input: &Vec<Machine>) -> u64 {
    let mut total_tokens = 0;
    for machine in input {
        let determinant = machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0;
        let num_a_undiv = machine.button_b.1 * machine.objective.0 - machine.button_b.0 * machine.objective.1;
        let num_b_undiv = -machine.button_a.1 * machine.objective.0 + machine.button_a.0 * machine.objective.1;

        println!("det: {determinant}, a: {num_a_undiv}, b: {num_b_undiv}");
        if (num_a_undiv % determinant == 0) && (num_b_undiv % determinant == 0) {
            // Can win with this machine!
            let num_a = num_a_undiv / determinant;
            let num_b = num_b_undiv / determinant;
            println!("Can win with {num_a}, {num_b}");
            total_tokens += (3 * num_a + num_b) as u64;
        }
        else {
            println!("Can't win");
        }
    }
    return total_tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_13_example.txt"} else { &args[1] };

    let input = get_input(&input_path);

    let total_tokens = get_token_count(&input);
    println!("Part one: {total_tokens}");

    let adjustment = 10000000000000;
    let mut input = input.clone();
    for machine in input.iter_mut() {
        machine.objective.0 += adjustment;
        machine.objective.1 += adjustment;
    }
    let total_tokens = get_token_count(&input);
    println!("Part two: {total_tokens}");

}