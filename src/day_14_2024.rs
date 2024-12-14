use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin, Read};
use regex::Regex;

#[derive(Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

fn get_input(input_path: &str) -> Vec<Robot> {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let mut robots = vec![];
    let buf = BufReader::new(input);
    let robot_re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

    for line in buf.lines().map(|l| l.unwrap()) {
        for (_, [p1, p2, v1, v2]) in robot_re.captures_iter(&line).map(|c| c.extract()) {
            let robot = Robot{
                position: (p1.parse().unwrap(), p2.parse().unwrap()),
                velocity: (v1.parse().unwrap(), v2.parse().unwrap())
            };
            robots.push(robot);
        }
    }

    return robots;
}

fn display_robots(robots: &Vec<Robot>, width: i32, height: i32) {
    let mut output = vec![vec![0; width as usize]; height as usize];
    // Populate output array
    for robot in robots {
        output[robot.position.1 as usize][robot.position.0 as usize] += 1;
    }

    // Display output
    for j in 0..height as usize{
        for i in 0..width as usize{
            if output[j][i] == 0 {
                print!(" ");
            }
            else {
                print!("{}", output[j][i]);
            }
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_14_example.txt"} else { &args[1] };
    let width = if args.len() < 2 { 11 } else { 101 };
    let height = if args.len() < 2 { 7 } else { 103 };

    let input = get_input(&input_path);

    let mut ne_quadrant = 0;
    let mut nw_quadrant = 0;
    let mut se_quadrant = 0;
    let mut sw_quadrant = 0;

    let mut robots_after_100 = input.clone();
    for robot in robots_after_100.iter_mut() {
        robot.position.0 += 100 * robot.velocity.0;
        robot.position.0 = ((robot.position.0 % width) + width) % width;
        robot.position.1 += 100 * robot.velocity.1;
        robot.position.1 = ((robot.position.1 % height) + height) % height;

        if robot.position.0 < width / 2 {
            if robot.position.1 < height / 2 {
                nw_quadrant += 1;
            }
            else if robot.position.1 > height / 2 {
                sw_quadrant += 1;
            }
        }
        else if robot.position.0 > width / 2 {
            if robot.position.1 < height / 2 {
                ne_quadrant += 1;
            }
            else if robot.position.1 > height / 2 {
                se_quadrant += 1;
            }
        }
    }
    let safety_score = ne_quadrant * se_quadrant * nw_quadrant * sw_quadrant;
    println!("Part one: {safety_score}");

    // Part two - search for a Christmas tree!
    let mut robots = input.clone();
    let mut seconds_elapsed = 0;
    loop {
        println!("Robots after {seconds_elapsed} seconds");
        display_robots(&robots, width, height);
        for robot in robots.iter_mut() {
            robot.position.0 += robot.velocity.0;
            robot.position.0 = ((robot.position.0 % width) + width) % width;
            robot.position.1 += robot.velocity.1;
            robot.position.1 = ((robot.position.1 % height) + height) % height;
        }
        seconds_elapsed += 1;
        stdin().read(&mut [0]).unwrap();
    }
}