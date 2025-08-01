use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn evaluate(&self, remaining_operands: Vec<u64>, result: u64) -> bool {

        if remaining_operands.is_empty() {
            return result == self.result;
        }
        
        let next_operand: u64 = remaining_operands[0];

        let mult_result = result * next_operand;
        let add_result = result + next_operand;
        let concatenation_result = (result.to_string() + &next_operand.to_string()).parse::<u64>().unwrap();


        if mult_result <= self.result {
            if self.evaluate(remaining_operands[1..].to_vec(), mult_result) {
                return true;
            }
        }
        if add_result <= self.result {
            if self.evaluate(remaining_operands[1..].to_vec(), add_result) {
                return true;
            }
        }
        if concatenation_result <= self.result {
            if self.evaluate(remaining_operands[1..].to_vec(), concatenation_result) {
                return true;
            }
        }
        false
    }
}

fn main() {

    const FILE_PATH: &str = "./input.txt";

    let mut equations: Vec<Equation> = Vec::new();

    // Open the text file
    if let Ok(lines) = read_lines(FILE_PATH) {
        lines.enumerate().for_each(|(x,line)| {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() == 2 {
                    println!("Processing line {}: {} | parts {:?}", x + 1, line, parts);
                    let result = parts[0].trim().parse::<u64>().unwrap();
                    let operands: Vec<u64> = parts[1]
                        .split_whitespace()
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect();
                    equations.push(Equation { result, operands });
                }
            }
        });
    }

    let final_sum: u64 = equations.iter().filter_map(
        |equation| {
            if equation.evaluate(equation.operands[1..].to_vec(), equation.operands[0]) {
                println!("Equation with result {} is valid: {:?}", equation.result, equation.operands);
                Some(equation.result)
            } else {
                if equation.operands.len() > 1 && equation.evaluate(equation.operands[2..].to_vec(), (equation.operands[0].to_string() + &equation.operands[1].to_string()).parse::<u64>().unwrap()) {
                    println!("Equation with result {} is valid after concatenation: {:?}", equation.result, equation.operands);
                    Some(equation.result)
                } else {
                    None
                }
            }
        }
    ).sum();

    println!("The sum of all valid equations is: {}", final_sum);
    

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}