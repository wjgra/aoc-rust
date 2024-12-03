use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input_data(input_path: &str) -> Vec<Vec<i64>> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    let arr = buf.lines()
        .map(|l| l.unwrap().trim().split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect())
        .collect();

    return arr;
}

fn check_record_is_safe(record: &Vec<i64>) -> bool{
    if record.len() == 1 {
        return true;
    }

    let direction = (record[1] - record[0]).signum();

    for i in 1..record.len() {
        let delta = record[i] - record[i - 1];
        let this_dir = delta.signum();
        if (this_dir == 0) || (this_dir != direction) {
            return false;
        }
        if delta.abs() > 3 {
            return false;
        }
    }

    return true;
}

fn check_dampened_record_is_safe(record: &Vec<i64>) -> bool{
    // If record is already safe, we are done
    if check_record_is_safe(&record) {
        return true;
    }

    // Check whether removing each entry makes it safe
    for i in 0..record.len() {
        let dampened_record = [&record[..i], &record[i+1..]].concat();
        if check_record_is_safe(&dampened_record) {
            return true;
        }
    }

    // Record cannot be made safe
    return false;
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 { "day_2_example.txt"} else { &args[1] };
    let data = get_input_data(input_path);

    // Part one - check if strictly monotonic and no level changes by more than three
    let mut safe_records = 0;
    for record in &data {
        if check_record_is_safe(&record) {
            safe_records += 1;
        }
    }
    println!("Safe records: {safe_records}");

    // Part two - check whether removing an entry makes a record satisfy part one
    let mut safe_records = 0;
    for record in &data {
        if check_dampened_record_is_safe(&record) {
            safe_records += 1;
        }
    }
    println!("Safe records: {safe_records}");
}