use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{result};
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
        || self.levels.is_empty()
    }

    fn is_really_safe(&self, threshold_min: i32, threshold_max: i32, desc: bool) -> bool {
        if self.is_safe(threshold_min, threshold_max) {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut new_levels = self.levels.clone();
            new_levels.remove(i);
            let temp_report = Report { levels: new_levels };
            if temp_report.is_safe(threshold_min, threshold_max) {
                return true;
            }
        }



        let error_index = self.levels.iter().zip(self.levels.iter().skip(1)).enumerate().find_map(|(i, (a, b))| {
            let diff = if desc { a - b } else { b - a };
            if diff < threshold_min || diff > threshold_max {
                return Some(i)
            }
            None
        });

        match error_index {
            Some(idx) => {
                if idx >= 3 {
                        let mut new_levels = self.levels.clone();
                        new_levels.remove(idx - 3);
                        let r3 = Report {
                            levels: new_levels
                        };
                        if r3.is_safe(threshold_min, threshold_max) {
                            return true;
                        }
                }
                if idx >= 2 {
                    let mut new_levels = self.levels.clone();
                    new_levels.remove(idx - 2);
                    let r2 = Report {
                        levels: new_levels
                    };
                    if r2.is_safe(threshold_min, threshold_max) {
                        return true;
                    }
                }

                if idx >= 1 {
                    let mut new_levels = self.levels.clone();
                    new_levels.remove(idx - 1);
                    let r1 = Report {
                        levels: new_levels
                    };
                    if r1.is_safe(threshold_min, threshold_max) {
                        return true;
                    }
                }

                let mut new_levels = self.levels.clone();
                new_levels.remove(idx);
                let r0 = Report {
                    levels: new_levels
                };
                if r0.is_safe(threshold_min, threshold_max) {
                    return true;
                }
                false
            },
            None => false
        }
    }

}




fn main() {

    const FILE_PATH: &str = "./input.txt";


    // let mut input_matrix: Vec<Report> = Vec::new();

    let mut safe_reports = 0;
    let mut really_safe_reports = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(line) = line {

                let report = Report::from_str(&line).unwrap();
                if report.is_safe(1, 3) {
                    safe_reports += 1;
                }
                if report.is_really_safe(1, 3, false) {
                    really_safe_reports += 1;
                } else if report.is_really_safe(1, 3, true) {
                    really_safe_reports += 1;
                }
            }
        }
    }
    println!("Number of safe reports: {}", safe_reports);
    println!("Number of really safe reports: {}", really_safe_reports);


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}