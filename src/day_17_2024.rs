use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Clone)]
struct RegisterState {
    a_value: u64,
    b_value: u64,
    c_value: u64
}

fn get_input(input_path: &str) -> (RegisterState, Vec<i8>) {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);
    let register_re = Regex::new(r"Register (.): (-?[0-9]+)").unwrap();
    let program_re = Regex::new(r"Program: (.*)").unwrap();

    let mut register_state = RegisterState {
        a_value: 0,
        b_value: 0,
        c_value: 0
    };

    let mut program_data = vec![];
    for line in buf.lines().map(|l| l.unwrap()) {
        for (_, [register, value]) in register_re.captures_iter(&line).map(|c| c.extract()) {
            println!("Initialised register {register} with value {value}");
            match register {
                "A" => register_state.a_value = value.parse::<u64>().unwrap(),
                "B" => register_state.b_value = value.parse::<u64>().unwrap(),
                "C" => register_state.c_value = value.parse::<u64>().unwrap(),
                _ => unreachable!()
            }
        }

        for (_, [input_data]) in program_re.captures_iter(&line).map(|c| c.extract()) {
            program_data = input_data.trim().split(',').map(|n| n.parse::<i8>().unwrap()).collect();
        }
    }

    return (register_state, program_data);
}

fn get_combo_operand(registers: &mut RegisterState, operand: i8) -> u64 {
    match operand {
        0 => return 0,
        1 => return 1,
        2 => return 2,
        3 => return 3,
        4 => return registers.a_value,
        5 => return registers.b_value,
        6 => return registers.c_value,
        7 => unreachable!(),
        _ => unreachable!()
    }
}

fn get_output(original_registers: &RegisterState, data: &Vec<i8>) -> Vec<i8> {

    let mut registers = original_registers.clone();
    let mut output = vec![];
    let mut program_counter = 0;

    while program_counter < data.len() {
        match data[program_counter] {
            0 => {
                // adv instruction
                registers.a_value = registers.a_value / 2_u64.pow(get_combo_operand(&mut registers, data[program_counter + 1]) as u32);
            },
            1 => {
                // bxl instruction
                registers.b_value ^= data[program_counter + 1] as u64;
            },
            2 => {
                // bst instruction
                registers.b_value = get_combo_operand(&mut registers, data[program_counter + 1]) % 8;
            },
            3 => {
                // jnz instruction
                if registers.a_value != 0 {
                    program_counter = data[program_counter + 1] as usize;
                    continue; // do not increment program counter
                }
            },
            4 => {
                // bxc instruction
                registers.b_value = registers.b_value ^ registers.c_value;
                // ignore operand
            },
            5 => {
                // out instruction
                let operand = get_combo_operand(&mut registers, data[program_counter + 1]) % 8;
                output.push(operand as i8);
            },
            6 => {
                // bdv instruction
                registers.b_value = registers.a_value / 2_u64.pow(get_combo_operand(&mut registers, data[program_counter + 1]) as u32);
            },
            7 => {
                // cdv instruction
                registers.c_value = registers.a_value / 2_u64.pow(get_combo_operand(&mut registers, data[program_counter + 1]) as u32);
            },
            _ => unreachable!()
        }
        program_counter += 2;
    }
    return output;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_17_example.txt"} else { &args[1] };
    let (registers, data) = get_input(&input_path);

    // Part one - get output of program
    let output = get_output(&registers, &data);

    let mut output_str = String::new();
    if output.len() > 0 {
        output_str += output[0].to_string().as_str();
    }
    for i in 1..output.len() {
        output_str += ",";
        output_str += output[i].to_string().as_str();
    }
    println!("Part one: {}", output_str);

    // Part two - find the smallest value of A which reproduces the input

    let total_digits = data.len();
    let mut to_process = vec![];

    to_process.push((0,0)); // (number of digits processed, current a-value)

    // If you consider each new digit in descending order, the value on top of the stack is the most
    // promising candidate.

    while !to_process.is_empty() {
        let (num_octal_digits_processed, current_a_value) = to_process.pop().unwrap();
        if num_octal_digits_processed == total_digits {
            println!("Part two: {current_a_value}");
            return;
        }
        for try_digit in (0..0o10  as u64).rev() {
            if try_digit == 0 && num_octal_digits_processed == 0 {
                continue; // Need at least a solution of full length
            }
            let a_increment = try_digit << (3 * (total_digits - 1 - num_octal_digits_processed));
            let try_a_value = current_a_value + a_increment;

            let try_registers = RegisterState{
                a_value: try_a_value,
                b_value: registers.b_value,
                c_value: registers.c_value
            };
            let output = get_output(&try_registers, &data);
            let comp_idx = total_digits - 1 - num_octal_digits_processed;
            if output[comp_idx] == data[comp_idx] {
                to_process.push((num_octal_digits_processed + 1, try_a_value));
            }
        }
    }
    println!("No solution found for part two!");
}