use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Read data from each column of the input file into a separate vector.
fn get_data_lists(path: &str) -> (Vec<i64>, Vec<i64>) {
    let file = File::open(path)
        .expect("Failed to open file");

    let buf = BufReader::new(file);


    let line_it: Vec<Vec<i64>> = buf.lines()
        .map(|l| l.unwrap().trim().split_whitespace()
            .map(|num| num.parse::<i64>().unwrap()).collect()).
        collect();

    let mut v1 = vec![];
    let mut v2 = vec![];
    for el in line_it {
        v1.push(el[0]);
        v2.push(el[1]);
    }
    return (v1, v2);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 { "input/day_1_example.txt"} else { &args[1] };

    let (mut first_list, mut second_list) = get_data_lists(&input_path);

    // Part one - taxicab distance between lists
    first_list.sort();
    second_list.sort();

    println!("Sorted data!");

    let mut total_diff = 0;
    for (a, b) in first_list.iter().zip(second_list.iter()) {
        total_diff += (a - b).abs();
    }

    println!("Total diff: {total_diff}");

    // Part two - similarity metric
    let mut similarity = 0;

    for a in first_list.iter() {
        let occurrences_in_second = second_list.iter()
            .filter(|&n| *n == *a).count();
        similarity += a * (occurrences_in_second as i64);
    }

    println!("Similarity: {similarity}");
}
