use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn get_input(input_path: &str) -> (Vec<Vec<bool>>, HashMap<char, Vec<(i32, i32)>>) {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);
    // Load input into an array
    let mut input_map = vec![];
    for line in buf.lines().map(|l| l.unwrap()) {
        let input_line: Vec<char> = line.chars().collect();
        input_map.push(input_line);
    }
    
    // Identify which antennas are present and populate lists of locations
    let output_map = vec![vec![false; input_map[0].len()]; input_map.len()];
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..input_map.len() {
        for j in 0..input_map[0].len() {
            let this_char = input_map[i][j];
            let next_pair = (i as i32, j as i32);
            match this_char {
                '.' => continue, // Not an antenna
                _ => {
                    if antennas.contains_key(&this_char) {
                        antennas.get_mut(&this_char).unwrap().push(next_pair);
                    }
                    else {
                        antennas.insert(this_char,vec![next_pair]);
                    }
                },
            }
        }
    }
    
    return (output_map, antennas);
}

fn is_inside_output_map(output_map: &Vec<Vec<bool>>,  pos: &(i32, i32)) -> bool{
    return pos.0 >= 0 && pos.0 < (output_map.len() as i32) && pos.1 >= 0 && pos.1 < (output_map[0].len() as i32);
}

fn add_to_output_map(output_map: &mut Vec<Vec<bool>>,  pos: &(i32, i32)) {
    if is_inside_output_map(output_map, pos) {
        output_map[pos.0 as usize][pos.1 as usize] = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_8_example.txt"} else { &args[1] };

    // Part one: find antinodes
    let (mut output_map, antennas) = get_input(input_path);

    for antennas_list in antennas.iter() {
        // Iterate over all pairs
        for i in 0..antennas_list.1.len() {
            for j in 0..i {
                let i_pos = antennas_list.1[i];
                let j_pos = antennas_list.1[j];
                let separation = (j_pos.0 - i_pos.0, j_pos.1 - i_pos.1);

                let i_antinode = (i_pos.0 - separation.0, i_pos.1 - separation.1);
                let j_antinode = (j_pos.0 + separation.0, j_pos.1 + separation.1);

                add_to_output_map(&mut output_map, &i_antinode);
                add_to_output_map(&mut output_map, &j_antinode);
            }
        }
    }

    let total_antinodes: u64 = output_map.iter()
    .map(|line| line.iter().filter(|&n| *n == true).count() as u64).sum();

    println!("Part one: {total_antinodes}");

    // Part two: antinodes are everywhere in line with antennas (don't need to erase output map as previous are all included)
    for antennas_list in antennas.iter() {
        // Iterate over all pairs
        for i in 0..antennas_list.1.len() {
            for j in 0..i {
                let i_pos = antennas_list.1[i];
                let j_pos = antennas_list.1[j];
                let separation = (j_pos.0 - i_pos.0, j_pos.1 - i_pos.1);

                // Divide separation by GCD to get all positions exactly in line
                let gcd = gcd::binary_u32(separation.0.abs() as u32, separation.1.abs() as u32) as i32;
                let separation = (separation.0 / gcd, separation.1 / gcd);
                
                // Consider all possible positions in positive and negative direction starting from i antenna
                let mut current_pos = i_pos;
                while is_inside_output_map(&output_map, &current_pos) {
                    output_map[current_pos.0 as usize][current_pos.1 as usize] = true;
                    current_pos.0 += separation.0;
                    current_pos.1 += separation.1;
                }

                let mut current_pos = i_pos;
                while is_inside_output_map(&output_map, &current_pos) {
                    output_map[current_pos.0 as usize][current_pos.1 as usize] = true;
                    current_pos.0 -= separation.0;
                    current_pos.1 -= separation.1;
                }

            }
        }
    }

    let total_antinodes: u64 = output_map.iter()
    .map(|line| line.iter().filter(|&n| *n == true).count() as u64).sum();

    println!("Part two: {total_antinodes}");

}