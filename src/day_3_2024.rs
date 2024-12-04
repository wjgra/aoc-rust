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

    // Part two - total result of enabled mul instructions
    let extended_input = String::from("do()") + &input_file;
    // println!("{extended_input}");
    let re = Regex::new(r"(.+?)mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total: i64 = 0;
    let mut enabled = true;

    let re_do_dont = Regex::new(r"do(.{0,3})\(\)").unwrap();
    for (_, [between, num1, num2]) in re.captures_iter(&extended_input).map(|c| c.extract()) {
        // println!("{between} {num1} {num2}");
        
        let dos = re_do_dont.captures_iter(&between).map(|c| c.extract());        
        let last_opt = dos.last();

        if last_opt.is_none() {
            // println!("None!");
        }
        else {
            let (_, [last]) = last_opt.unwrap();
            // println!("{last}");
            if last == "n't" {
                enabled = false;
            }
            else if last == "" {
                enabled = true;
            }
        }


        // for (_, [nt]) in re_do_dont.captures_iter(&between) {

        // }

        // for (_, [nt]) in dos.map(|c| c.extract()) {
        //     println!("{nt}");
        // }

        // if nt == "n't" {
        //     enabled = false;
        // }
        // else if nt == "" {
        //     enabled = true;
        // }
        if enabled {
            total += (num1.parse::<i64>().unwrap()) * (num2.parse::<i64>().unwrap());
        }
    }
    println!("Total: {total}");

    let re = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\))|(do\(\))|(don't\(\))").unwrap();


    
}