use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input_data(input_path: &str) -> Vec<Vec<char>> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    let arr = buf.lines()
        .map(|l| l.unwrap().chars().collect()).collect();

    return arr;
}

fn check_xmas_present(data: &Vec<Vec<char>>, i: usize, j: usize, x_dir: i8, y_dir: i8) -> bool {
    let width = data.len();
    let height = data[0].len();
    let test_str = "XMAS";

    // Starting a i, j, proceed in dir, checking str values
    let mut pos_x = i as i64;
    let mut pos_y = j as i64;
    for ch in test_str.chars() {
        if (pos_x < 0) || (pos_x >= width as i64) || (pos_y < 0) || (pos_y >= height as i64) {
            return false;
        }
        if data[pos_x as usize][pos_y as usize] != ch {
            return false;
        }
        else {
            pos_x += x_dir.signum() as i64;
            pos_y += y_dir.signum() as i64;
        }
    }
    return true
}

// Upper left corner is zeroth, upper right is 1st, ...
fn index_to_corner(idx: usize) -> (i8, i8) {
    if idx % 4 == 0 {
        return (-1, -1);
    }
    if idx % 4 == 1 {
        return (-1, 1);
    }
    if idx % 4 == 2 {
        return (1, 1);
    }
    if idx % 4 == 3 {
        return (1, -1);
    }
    return (0,0); // err
}

// Here, dir_idx indicates the first M (order M-M-S-S clockwise)
fn check_x_mas_present(data: &Vec<Vec<char>>, i: usize, j: usize, dir_idx: usize) -> bool{
    let width = data.len();
    let height = data[0].len();

    if data[i][j] != 'A' {
        return false;
    }

    // Not possible for X-MAS to fit in the grid
    if (i == 0) || (j == 0) || (i == width - 1) || (j == height - 1) {
        return false;
    }
    
    let test_str= vec!['M', 'M', 'S', 'S'];
    for idx in 0..4 {
        let dir = index_to_corner(dir_idx + idx);
        if test_str[idx] != data[((i as i64) + (dir.0 as i64)) as usize][((j as i64) + (dir.1 as i64)) as usize] {
            return false;
        }
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 { "day_4_example.txt"} else { &args[1] };
    let data = get_input_data(input_path);

    let width = data.len();
    let height = data[0].len();

    // Part one - count occurrences of XMAS in the grid
    let mut count = 0;
    for i in 0..width {
        for j in 0..height {
            // Check for each direction of XMAS
            if check_xmas_present(&data, i, j, 0, 1) { // N
                count += 1;
            }
            if check_xmas_present(&data, i, j, 0, -1) { // S
                count += 1;
            }
            if check_xmas_present(&data, i, j, 1, 0) { // E
                count += 1;
            }
            if check_xmas_present(&data, i, j, -1, 0) { // W
                count += 1;
            }
            if check_xmas_present(&data, i, j, 1, 1) { // NE
                count += 1;
            }
            if check_xmas_present(&data, i, j, 1, -1) { // SE
                count += 1;
            }
            if check_xmas_present(&data, i, j, -1, 1) { // NW
                count += 1;
            }
            if check_xmas_present(&data, i, j, -1, -1) { // SW
                count += 1;
            }
        }
    }
    println!("Occurrences of XMAS: {count}");

    // Part two - count occurrences of X-MAS in the grid
    let mut count = 0;
    for i in 0..width {
        for j in 0..height {
            // Check for each direction of X-MAS
            if check_x_mas_present(&data, i, j, 0) {
                count += 1;
            }
            if check_x_mas_present(&data, i, j, 1) {
                count += 1;
            }
            if check_x_mas_present(&data, i, j, 2) {
                count += 1;
            }
            if check_x_mas_present(&data, i, j, 3) {
                count += 1;
            }
        }
    }
    println!("Occurrences of X_MAS: {count}");
}