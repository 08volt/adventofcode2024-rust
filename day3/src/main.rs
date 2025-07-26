use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::{zip, Map};
use std::path::Path;
use std::result;
use std::str::FromStr;


struct Report {
    levels: Vec<i32>
}

impl FromStr for Report {
    type Err = io::Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let levels: Vec<i32> = s.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Report { levels })
    }
}

impl Report {
    fn is_safe(&self, threshold_min: i32, threshold_max: i32) -> bool {
        self.levels.windows(2).all(|pair| threshold_min <= (pair[1] - pair[0]) && (pair[1] - pair[0]) <= threshold_max)
        || self.levels.windows(2).all(|pair| threshold_min <= (pair[0] - pair[1]) && (pair[0] - pair[1]) <= threshold_max)
        || self.levels.len() == 1
    }
}




fn main() {

    const FILE_PATH: &str = "./input.txt";


    // let mut input_matrix: Vec<Report> = Vec::new();

    let mut safe_reports = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(line) = line {

                let report = Report::from_str(&line).unwrap();
                if report.is_safe(1, 3) {
                    safe_reports += 1;
                }
            }
        }
    }
    println!("Number of safe reports: {}", safe_reports);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}