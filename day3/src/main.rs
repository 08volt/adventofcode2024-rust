use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::path::Path;


fn main() {

    const FILE_PATH: &str = "./input.txt";

    
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    let mut enabled = true;
    // Open the text file
    if let Ok(lines) = read_lines(FILE_PATH) {
        let final_result: u32 = lines.map(|line| {
            let mut result = 0;
            if let Ok(line) = line {
                
                for cap in re.captures_iter(&line) {
                    if let Some(mul_match) = cap.get(0) {
                        if mul_match.as_str().starts_with("mul") && enabled{
                            let x: u32 = cap[1].parse().unwrap();
                            let y: u32 = cap[2].parse().unwrap();
                            result += x * y;
                        } else if mul_match.as_str() == "don't()" {
                            enabled = false;
                            println!("Found: don't()");
                        } else if mul_match.as_str() == "do()" {
                            enabled = true;
                            println!("Found: do()");
                        }
                    }
                }
            }
            return result
        }).sum();
        println!("The final result is: {}", final_result);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}