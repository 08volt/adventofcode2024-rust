use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    const FILE_PATH: &str = "./input.txt";

    let mut before_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut producing_lists: Vec<Vec<i32>> = Vec::new();

    // Open the text file
    if let Ok(lines) = read_lines(FILE_PATH) {
        // Iterate through each line in the file
        for line in lines {
            if let Ok(ip) = line {
                // Split the line into parts
                let parts: Vec<&str> = ip.split('|').collect();
                if parts.len() == 2 {
                    // Parse the first two parts as integers
                    if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                        // Get or create the set for the first integer
                        let set = before_map.entry(a).or_insert_with(HashSet::new);
                        // Insert the second integer into the set
                        set.insert(b);
                    }
                } else {
                    let numbers = ip.split(',')
                        .filter_map(|s| s.trim().parse::<i32>().ok())
                        .collect::<Vec<i32>>();
                    if numbers.len() < 2 {
                        continue; // Skip lists with less than 2 numbers
                    }
                    producing_lists.push(numbers);
                }
            }
        }
    }

    println!("Before map: {:?}", before_map);

    let valid_lists = producing_lists.iter()
        .filter(|list| {
            let mut seen: HashSet<i32> = HashSet::new();
            list.iter().all(|&num| {
                seen.insert(num);
                let before_set = before_map.get(&num);
                if let Some(set) = before_set {
                    set.intersection(&seen).count() == 0
                } else {
                    true
                }
            })
        })
        .collect::<Vec<&Vec<i32>>>();

    let mid_numbers_sum: i32 = valid_lists.iter()
        .flat_map(|list| list.get(list.len() / 2))
        .sum();
    println!("Sum of middle numbers of valid lists: {}", mid_numbers_sum);

    let invalid_lists = producing_lists.iter()
        .filter(|list| {
            let mut seen: HashSet<i32> = HashSet::new();
            list.iter().any(|&num| {
                seen.insert(num);
                let before_set = before_map.get(&num);
                if let Some(set) = before_set {
                    set.intersection(&seen).count() > 0
                } else {
                    false
                }
            })
        })
        .collect::<Vec<&Vec<i32>>>();

    
    let fix_invalid_lists: Vec<Vec<i32>> = invalid_lists.iter()
        .map(|list| {
            let mut fixed_list: Vec<i32> = Vec::new();
            list.iter().for_each(|&num| {
                fixed_list.push(num);
                let before_set = before_map.get(&num);
                if let Some(set) = before_set {
                    let move_numbers: Vec<i32> = fixed_list.iter().filter(|&x| set.contains(x)).cloned().collect();
                    if move_numbers.len() > 0 {
                        move_numbers.iter().for_each(|&num| {
                            if let Some(index) = fixed_list.iter().position(|&x| x == num) {
                                fixed_list.remove(index);
                                
                            }
                        });
                        fixed_list.extend(move_numbers);
                    }
                }
            });
            fixed_list
        })
        .collect();

    let mid_numbers_sum: i32 = fix_invalid_lists.iter()
        .flat_map(|list| list.get(list.len() / 2))
        .sum();
    println!("Sum of middle numbers of fixed lists: {}", mid_numbers_sum);

    let still_invalid_lists = fix_invalid_lists.iter()
        .filter(|list| {
            let mut seen: HashSet<i32> = HashSet::new();
            list.iter().any(|&num| {
                seen.insert(num);
                let before_set = before_map.get(&num);
                if let Some(set) = before_set {
                    set.intersection(&seen).count() > 0
                } else {
                    false
                }
            })
        })
        .collect::<Vec<&Vec<i32>>>();

    println!("Still invalid lists: {:?}", still_invalid_lists);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}