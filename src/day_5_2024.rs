use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use multimap::MultiMap;

fn get_input(input_path: &str) -> (MultiMap<u32, u32>, MultiMap<u32, u32>, Vec<Vec<u32>>) {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    let mut read_all_rules = false;

    let mut page_lists: Vec<Vec<u32>> = vec![];
    let mut after_rules = MultiMap::new();
    let mut before_rules = MultiMap::new();

    for line in buf.lines().map(|l| l.unwrap()) {
        if line == "" {
            read_all_rules = true;
            continue;
        }
        if read_all_rules {
            // Read page lists
            let this_list = line.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect(); 
            page_lists.push(this_list);
        } else {
            // Read rules
            let this_rule: Vec<u32> = line.split('|')
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            
            after_rules.insert(this_rule[0], this_rule[1]);
            before_rules.insert(this_rule[1], this_rule[0]);
        }

    }
    return (after_rules, before_rules, page_lists);
}

/*
Multimaps: 
after_rules <pre, post>
before_rules <post, pre>
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 { "input/day_5_example.txt"} else { &args[1] };
    let (after_rules, before_rules, page_lists) = get_input(input_path);

    // Part one: total of middle numbers in correctly ordered lists
    let mut total_pt1 = 0 as u64;

    // Part two: total of middle numbers in incorrectly ordered lists after reordering
    let mut total_pt2 = 0 as u64;

    for list in &page_lists {
        // For each list, check whether each number is correctly printed
        let mut correctly_printed = true;
        for i in 0..list.len() {
            let current_num = list[i];
            // Check whether numbers before current are printed correctly
            let after_current_rules = after_rules.get_vec(&current_num);
            
            match after_current_rules {
                Some(nums) => {
                    for j in 0..i {
                        // If rule says current must be before jth, list is not correct
                        if nums.contains(&list[j]) {
                            correctly_printed = false;
                            break;
                        } 
                    }
                },
                None => {
                    // No rules preventing list from being printed correctly
                }
            }

            // Check whether numbers after current are printed correctly
            let before_current_rules = before_rules.get_vec(&current_num);
            
            match before_current_rules {
                Some(nums) => {
                    for j in (i + 1)..list.len() {
                        // If rule says current must be after jth, list is not correct
                        if nums.contains(&list[j]) {
                            correctly_printed = false;
                            break;
                        } 
                    }
                },
                None => {
                    // No rules preventing list from being printed correctly
                }
            }

            if correctly_printed == false {
                break;
            }
        }

        if correctly_printed {
            // Add middle number to total
            total_pt1 += list[(list.len() - 1) / 2] as u64;
        }
        else {
            let compare = |a: &u32, b: &u32| -> std::cmp::Ordering {
                let after_a = after_rules.get_vec(a);
                if after_a.is_some() && after_a.unwrap().contains(b){
                    return std::cmp::Ordering::Less;
                }

                let before_a = before_rules.get_vec(a);
                if before_a.is_some() && before_a.unwrap().contains(b){
                    return std::cmp::Ordering::Greater;
                }

                return std::cmp::Ordering::Equal;
            };

            let mut sorted_list = list.to_vec();
            sorted_list.sort_unstable_by(compare);

            total_pt2 += sorted_list[(sorted_list.len() - 1) / 2] as u64;   
        }
    }
    println!("Part one total: {total_pt1}");
    println!("Part two total: {total_pt2}");
}