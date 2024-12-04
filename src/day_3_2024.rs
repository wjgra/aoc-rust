use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 { "day_3_example.txt"} else { &args[1] };

    let input_file = std::fs::read_to_string(input_path).unwrap();

    // Part one - total result of mul instructions
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total: i64 = 0;
    for (_, [num1, num2]) in re.captures_iter(&input_file).map(|c| c.extract()) {
        total += (num1.parse::<i64>().unwrap()) * (num2.parse::<i64>().unwrap());
    }

    println!("Total: {total}");


    // Part two - mul may be enabled/disabled
    let re_any = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))").unwrap();
    let re_mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut enabled = true;
    let mut total = 0;
    for (_,[any]) in re_any.captures_iter(&input_file).map(|c| c.extract()) {
        if any == "do()" {
            enabled = true;
        }
        else if any == "don't()" {
            enabled = false;
        }
        else if enabled {
            for (_, [num1, num2]) in re_mul.captures_iter(any).map(|c| c.extract()) {
                total += (num1.parse::<i64>().unwrap()) * (num2.parse::<i64>().unwrap());
            }
        }
    }
    println!("Total: {total}");
}