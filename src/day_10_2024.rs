use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, VecDeque};

type GridPos = (usize, usize);

fn get_input(input_path: &str) -> (HashSet<GridPos>, HashSet<GridPos>, Vec<Vec<i8>>) {
    println!("Loading input at {input_path}");
    let input = File::open(input_path)
        .expect("Failed to open file");

    let buf = BufReader::new(input);

    // Load input into an array
    let mut height_map = vec![];
    let mut high_points = HashSet::new();
    let mut low_points = HashSet::new();
    for line in buf.lines().map(|l| l.unwrap()) {
        let input_line: Vec<i8> = line.chars().map(|n| n.to_digit(10).unwrap() as i8).collect();
        
        // Get high and low points
        for j in 0..input_line.len() {
            let i = height_map.len();
            match input_line[j] {
                0 => low_points.insert((i, j)),
                9 => high_points.insert((i, j)),
                _ => continue
            };
        }
        height_map.push(input_line);
    }

    return (high_points, low_points, height_map);
}

// fn get_two_refs<'a>(arr: &'a mut Vec<Vec<Vec<GridPos>>>, pt_1: &GridPos, pt_2: &GridPos) -> (&'a mut Vec<GridPos>, &'a mut Vec<GridPos>) {
    // Use split_at_mut to get references to two different points in an array...
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = if args.len() < 2 { "input/day_10_example.txt" } else { &args[1] };

    let (high_points, low_points, height_map) = get_input(&input_path);

    // Part one - find the sum of high points reachable at each low point
    // Part two - find the sum of paths to high points from each low point

    // A map of high points reachable at each point in the grid
    let mut reachable_map = vec![vec![vec![]; height_map[0].len()]; height_map.len()];
    // let mut paths_map= vec![vec![0; height_map[0].len()]; height_map.len()];
    let mut points_to_process = VecDeque::new(); // now I've separated the parts, can be a vector again...

    // Start with high points, which are reachable from themselves
    for pt in &high_points {
        reachable_map[pt.0][pt.1].push(*pt);
        // paths_map[pt.0][pt.1] = 1;
        points_to_process.push_front(*pt);
    }

    // DFS to find paths from high points to low points
    while !points_to_process.is_empty() {
        let this_pt = points_to_process.pop_back().unwrap();
        let this_height = height_map[this_pt.0][this_pt.1];
        // println!("Processing {}, {} of height {}", this_pt.0, this_pt.1, this_height);
        let target_height = this_height - 1;
        
        // Update neighbours, unless we're already at a low point
        if this_height > 0 {
            let mut target_pts: Vec<GridPos> = vec![];

            if this_pt.0 > 0 {
                target_pts.push((this_pt.0 - 1, this_pt.1));
            }
            if this_pt.0 < height_map.len() - 1 {
                target_pts.push((this_pt.0 + 1, this_pt.1));

            }
            if this_pt.1 > 0 {
                target_pts.push((this_pt.0, this_pt.1 - 1));

            }
            if this_pt.1 < height_map[0].len() - 1 {
                target_pts.push((this_pt.0, this_pt.1 + 1));

            }

            for target_pt in &target_pts {
                if height_map[target_pt.0][target_pt.1] == target_height {
                    // println!("Added {}, {} to points to process", target_pt.0, target_pt.1);
                    let reachable_to_add = reachable_map[this_pt.0][this_pt.1].clone(); // This is bad! Work out how to use split_at_mut()...
                    // let paths_to_add = paths_map[this_pt.0][this_pt.1].clone(); // Cloning a copyable type?

                    // paths_map[target_pt.0][target_pt.1] += paths_to_add;


                    for pt in &reachable_to_add {
                        if reachable_map[target_pt.0][target_pt.1].contains(pt) == false {
                            reachable_map[target_pt.0][target_pt.1].push(*pt);
                            // paths_map[target_pt.0][target_pt.1] = paths_to_add;
                            // paths_map[target_pt.0][target_pt.1] += 1;
                        }
                        else {
                            // paths_map[target_pt.0][target_pt.1] += 1;
                        }

                        // if paths_map[target_pt.0][target_pt.1] == 0 {
                        //     paths_map[target_pt.0][target_pt.1] = paths_to_add;
                        // }
                        // else {
                        //     paths_map[target_pt.0][target_pt.1] += paths_to_add;
                        // }
                        
                    }
                    // println!("Leaving {}, {} with path num {}", this_pt.0, this_pt.1, paths_map[this_pt.0][this_pt.1]);
                    points_to_process.push_front(*target_pt);
                }
            }            
        }
    }

    // Calculate sum of high points accessible from low points
    let mut total_score = 0;

    // let mut total_paths = 0;
    for pt in &low_points{
        // println!("Point at {}, {} has: {:?}", pt.0, pt.1, reachable_map[pt.0][pt.1]);
        // println!("Point at {}, {} has {}", pt.0, pt.1, paths_map[pt.0][pt.1]);
        total_score += reachable_map[pt.0][pt.1].len();
        // total_paths += paths_map[pt.0][pt.1];
    }
    println!("Part one: {total_score}");

    // Part two

     // A map of high points reachable at each point in the grid
    //  let mut reachable_map = vec![vec![vec![]; height_map[0].len()]; height_map.len()];
     let mut paths_map= vec![vec![0; height_map[0].len()]; height_map.len()];
     let mut points_to_process = VecDeque::new();
 
     // Start with high points, which are reachable from themselves
     for pt in &high_points {
        //  reachable_map[pt.0][pt.1].push(*pt);
         paths_map[pt.0][pt.1] = 1;
         points_to_process.push_front(*pt);
     }
 
     // DFS to find paths from high points to low points
     while !points_to_process.is_empty() {
         let this_pt = points_to_process.pop_back().unwrap();
         let this_height = height_map[this_pt.0][this_pt.1];
         println!("Processing {}, {} of height {}", this_pt.0, this_pt.1, this_height);
         println!("Vec: {:?}", points_to_process);
         let target_height = this_height - 1;
         
         // Update neighbours, unless we're already at a low point
         if this_height > 0 {
             let mut target_pts: Vec<GridPos> = vec![];
 
             if this_pt.0 > 0 {
                 target_pts.push((this_pt.0 - 1, this_pt.1));
             }
             if this_pt.0 < height_map.len() - 1 {
                 target_pts.push((this_pt.0 + 1, this_pt.1));
 
             }
             if this_pt.1 > 0 {
                 target_pts.push((this_pt.0, this_pt.1 - 1));
 
             }
             if this_pt.1 < height_map[0].len() - 1 {
                 target_pts.push((this_pt.0, this_pt.1 + 1));
 
             }

             // consider filter targets by == target height

             for target_pt in &target_pts {
                 if height_map[target_pt.0][target_pt.1] == target_height {
                     // println!("Added {}, {} to points to process", target_pt.0, target_pt.1);
                    //  let reachable_to_add = reachable_map[this_pt.0][this_pt.1].clone(); // This is bad! Work out how to use split_at_mut()...
                     let paths_to_add = paths_map[this_pt.0][this_pt.1].clone(); // Cloning a copyable type?
 
                    //  paths_map[target_pt.0][target_pt.1] += paths_to_add;
                    // if paths_map[target_pt.0][target_pt.1] == 0 {
                        paths_map[target_pt.0][target_pt.1] += 1;
                        println!("Updating path at {}, {} to {}", target_pt.0, target_pt.1, paths_map[target_pt.0][target_pt.1]);
                    // }
                    // else {
                    //     paths_map[target_pt.0][target_pt.1] += 1;
                    // }
 
 
                    //  for pt in &reachable_to_add {
                    //      if reachable_map[target_pt.0][target_pt.1].contains(pt) == false {
                    //          reachable_map[target_pt.0][target_pt.1].push(*pt);
                    //          // paths_map[target_pt.0][target_pt.1] = paths_to_add;
                    //          // paths_map[target_pt.0][target_pt.1] += 1;
                    //      }
                    //      else {
                    //          // paths_map[target_pt.0][target_pt.1] += 1;
                    //      }
 
                    //      // if paths_map[target_pt.0][target_pt.1] == 0 {
                    //      //     paths_map[target_pt.0][target_pt.1] = paths_to_add;
                    //      // }
                    //      // else {
                    //      //     paths_map[target_pt.0][target_pt.1] += paths_to_add;
                    //      // }
                         
                    //  }
                     
                     points_to_process.push_front(*target_pt);
                 }
             }       
             println!("Leaving {}, {} with path num {}", this_pt.0, this_pt.1, paths_map[this_pt.0][this_pt.1]);     
         }
     }


    let mut total_paths = 0;
    for pt in &low_points{
        // println!("Point at {}, {} has: {:?}", pt.0, pt.1, reachable_map[pt.0][pt.1]);
        println!("Point at {}, {} has {}", pt.0, pt.1, paths_map[pt.0][pt.1]);
        total_paths += paths_map[pt.0][pt.1];
    }





    println!("Part two: {total_paths}");
    
}