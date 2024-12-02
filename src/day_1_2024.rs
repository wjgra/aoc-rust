use std::env;
// use std::iter;
// use std::iter::zip;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input_data(input_path: &str) -> Vec<Vec<i64>> { // Ideally this would be a Vec<(i64, i64)>, but I haven't worked that out yet...
    // Load the data from file
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to load input file file");

    let buf = BufReader::new(input);

    let arr/* : Vec<(i64, i64)> */ = buf.lines()
        .map(|l| l.unwrap().split("   ")
            .map(|number| number.parse::<i64>().unwrap())
                .collect())
                .collect();

    
    return arr;
}


// fn get_data_lists(path: &str) -> (Vec<i64>, Vec<i64>) {
//     let file = File::open(path)
//         .expect("Failed to open file");

//     let buf = BufReader::new(file);


//     let line_it = buf.lines()
//         .map(|l| l.unwrap().split_whitespace()
//             .map(|num| num.parse::<i64>().unwrap())
//             .take(2))
//         .collect();

//     let (v1, v2) = zip(line_it[0], line_it[1]);

//     let (v1, v2) = buf.lines()
//         .map(|l| l.unwrap().split_whitespace()
//             .map(|num| num.parse::<i64>().unwrap())
//             .take(2)
//             .map(|it| (it, it.next()))
//             .take(1)
//             .collect())
//         .collect();

// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 { "day_1_example.txt"} else { &args[1] };
    let data = get_input_data(input_path);

    // let data = get_input_data("day_1_example.txt");
    let mut first_list: Vec<i64> = vec![];
    let mut second_list: Vec<i64> = vec![];

    // This is not idiomatic in the slightest...
    for el in data {
        first_list.push(el[0]);
        second_list.push(el[1]);
    }

    first_list.sort();
    second_list.sort();

    println!("Sorted data!");

    let mut total_diff = 0;
    for (a, b) in first_list.iter().zip(second_list.iter()) {
        total_diff += (a - b).abs();
    }

    // Part one
    println!("Total diff: {total_diff}");

    let mut similarity = 0;

    for a in first_list.iter() {
        let occurrences_in_second = second_list.iter()
            .filter(|&n| *n == *a).count();
        similarity += a * (occurrences_in_second as i64);
    }

    // Part two
    println!("Similarity: {similarity}");

}
