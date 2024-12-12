use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input(input_path: &str) -> Vec<Vec<char>> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    return buf.lines().map(|l| l.unwrap().chars().collect()).collect();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_12_example.txt"} else { &args[1] };

    let input = get_input(&input_path);

    // Part one - total price of the plots by perimeter
    // Part two - total price of the plots by number of sides
    let i_dim = input.len();
    let j_dim = input[0].len();
    let mut visited = vec![vec![false; j_dim]; i_dim];

    let mut price = 0;
    let mut price_reduced = 0;
    for i in 0..i_dim {
        for j in 0..j_dim {
            if visited[i][j] {
                continue;
            }
            // Start a new region
            let region_plant = input[i][j];
            let mut area = 0;
            let mut perimeter = 0;
            let mut corners = 0; // Number of corners == number of sides

            let mut to_process = vec![(i, j)];
            visited[i][j] = true;

            while !to_process.is_empty() {
                area += 1;
                let (this_i, this_j) = to_process.pop().unwrap();

                // Add any neighbours
                let mut targets = vec![];
                if this_i > 0 {
                    targets.push((this_i - 1, this_j));
                }
                else {
                    perimeter += 1;
                }
                if this_i < i_dim - 1 {
                    targets.push((this_i + 1, this_j));
                }
                else {
                    perimeter += 1;
                }
                if this_j > 0 {
                    targets.push((this_i, this_j - 1));
                }
                else {
                    perimeter += 1;
                }
                if this_j < j_dim - 1 {
                    targets.push((this_i, this_j + 1));
                }
                else {
                    perimeter += 1;
                }
                for (target_i, target_j) in targets {
                    if input[target_i][target_j] != region_plant {
                        perimeter += 1;
                    }
                    else if visited[target_i][target_j] == false {
                        to_process.push((target_i, target_j));
                        visited[target_i][target_j] = true;
                    }
                }

                // Count corners - there must be a neater way!
                // NW
                if (this_i == 0 && (this_j == 0 || input[0][this_j - 1] != region_plant)) || // in first row
                   (this_j == 0 && input[this_i - 1][0] != region_plant) || // in first column
                   (this_i > 0 && this_j > 0 && input[this_i - 1][this_j] != region_plant && input[this_i][this_j - 1] != region_plant) || // convex
                   (this_i > 0 && this_j > 0 && input[this_i - 1][this_j] == region_plant && input[this_i][this_j - 1] == region_plant && input[this_i - 1][this_j - 1] != region_plant) { // concave
                    corners += 1;
                }
                // NE
                if (this_i == 0 && (this_j == (j_dim - 1) || input[0][this_j + 1] != region_plant)) || // in first row
                   (this_j == (j_dim - 1) && input[this_i - 1][(j_dim - 1)] != region_plant) || // in last column
                   (this_i > 0 && this_j < (j_dim - 1) && input[this_i - 1][this_j] != region_plant && input[this_i][this_j + 1] != region_plant) || // convex
                   (this_i > 0 && this_j < (j_dim - 1) && input[this_i - 1][this_j] == region_plant && input[this_i][this_j + 1] == region_plant && input[this_i - 1][this_j + 1] != region_plant) { // concave
                    corners += 1;
                }
                // SW
                if (this_i == (i_dim - 1) && (this_j == 0 || input[this_i][this_j - 1] != region_plant)) || // in last row
                   (this_j == 0 && input[this_i + 1][0] != region_plant) || // in first column
                   (this_i < (i_dim - 1) && this_j > 0 && input[this_i + 1][this_j] != region_plant && input[this_i][this_j - 1] != region_plant) || // convex
                   (this_i < (i_dim - 1) && this_j > 0 && input[this_i + 1][this_j] == region_plant && input[this_i][this_j - 1] == region_plant && input[this_i + 1][this_j - 1] != region_plant) { // concave
                    corners += 1;
                }
                // SE
                if (this_i == (i_dim - 1) && (this_j == (j_dim - 1) || input[this_i][this_j + 1] != region_plant)) || // in last row
                   (this_j == (j_dim - 1) && input[this_i + 1][this_j] != region_plant) || // in last column
                   (this_i < (i_dim - 1) && this_j < (j_dim - 1) && input[this_i + 1][this_j] != region_plant && input[this_i][this_j + 1] != region_plant) || // convex
                   (this_i < (i_dim - 1) && this_j < (j_dim - 1) && input[this_i + 1][this_j] == region_plant && input[this_i][this_j + 1] == region_plant && input[this_i + 1][this_j + 1] != region_plant) { // concave
                    corners += 1;
                }
            }
            price += area * perimeter;
            price_reduced += area * corners;
        }
    }
    println!("Part one: {price}");
    println!("Part two: {price_reduced}");
}