use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    const FILE_PATH: &str = "./input.txt";

    let mut trail_map: HashMap<(i64, i64), i64> = HashMap::new();

    // Open the text file
    if let Ok(lines) = read_lines(FILE_PATH) {
        lines.enumerate().for_each(|(x,line)| {
            if let Ok(line) = line {
                line.chars().enumerate().for_each(|(y, c)| {
                    trail_map.insert((x.try_into().unwrap(),y.try_into().unwrap()), c.to_digit(10).unwrap().into());
                });
            }
        });
    }

    
   let result: usize = trail_map.iter().map( |((x,y), value) | {
        if *value == 0 {
            let trails = trail_heads(&trail_map, x.clone(), y.clone(), -1);
            return trails.len();
        }
        return 0
    }).sum();

    println!("Result Part 1: {}", result);

    let result_part2: u64 = trail_map.iter().map( |((x,y), value) | {
        if *value == 0 {
            return distinct_trail_heads(&trail_map, x.clone(), y.clone(), -1);
        }
        return 0
    }).sum();

    println!("Result Part 2: {}", result_part2)
}

fn trail_heads(trail_map: &HashMap<(i64, i64), i64>, x: i64, y: i64, prev_value: i64) -> HashSet<(i64, i64)> {
    let mut result: HashSet<(i64, i64)> = HashSet::new();

    let curr_value = trail_map.get(&(x.try_into().unwrap(), y.try_into().unwrap()));
    
    match curr_value {
        Some(v) => {
            if v == &(prev_value + 1) {
                if v == &9 {
                    result.insert((x.try_into().unwrap(),y.try_into().unwrap()));
                } else {
                    let left = trail_heads(trail_map, x-1, y, v.clone());
                    let up = trail_heads(trail_map, x, y-1, v.clone());
                    let right = trail_heads(trail_map, x+1, y, v.clone());
                    let down = trail_heads(trail_map, x, y+1, v.clone());
                    result.extend(left);
                    result.extend(up);
                    result.extend(right);
                    result.extend(down);
                }
                
            }
            return result
            
        },
        None => result,
    }
}

fn distinct_trail_heads(trail_map: &HashMap<(i64, i64), i64>, x: i64, y: i64, prev_value: i64) -> u64 {
    
    let curr_value = trail_map.get(&(x.try_into().unwrap(), y.try_into().unwrap()));
    
    match curr_value {
        Some(v) => {
            if v == &(prev_value + 1) {
                if v == &9 {
                    1
                } else {
                    let left = distinct_trail_heads(trail_map, x-1, y, v.clone());
                    let up = distinct_trail_heads(trail_map, x, y-1, v.clone());
                    let right = distinct_trail_heads(trail_map, x+1, y, v.clone());
                    let down = distinct_trail_heads(trail_map, x, y+1, v.clone());
                    left + up + right + down
                }
            }  else {
                0
            }          
        },
        None => 0,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}