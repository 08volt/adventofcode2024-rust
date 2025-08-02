use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result;



fn main() {

    const FILE_PATH: &str = "./input.txt";

    let mut letters_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut height: isize = 0;
    let mut width: isize = 0;

    // Open the text file
    if let Ok(lines) = read_lines(FILE_PATH) {
        lines.enumerate().for_each(|(x,line)| {
            if let Ok(line) = line {
                line.chars().enumerate().for_each(|(y, c)| {
                    if c != '.' {
                        letters_positions.entry(c).or_default().push((x, y));
                    }
                    height = height.max((x + 1).try_into().unwrap());
                    width = width.max((y + 1).try_into().unwrap());
                });
            }
        });
    }

    let mut same_letter_couples: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    letters_positions.iter().for_each(|(_, positions)| {
        if positions.len() > 1 {
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let pos1 = positions[i];
                    let pos2 = positions[j];
                    same_letter_couples.insert((pos1, pos2));
                    same_letter_couples.insert((pos2, pos1));
                }
            }
        }
    });

    let externsions: HashSet<(isize, isize)> = same_letter_couples.iter().flat_map(|(a, b)| {
        let diff_x = a.0 as isize - b.0 as isize;
        let diff_y = a.1 as isize - b.1 as isize;

        // let result_x = a.0 as isize + diff_x;
        // let result_y = a.1 as isize + diff_y;

        let mut result_x = a.0 as isize;
        let mut result_y = a.1 as isize;

        let mut results = Vec::new();

        while result_x >= 0 && result_y >= 0 && result_x < height && result_y < width {
            results.push((result_x, result_y));
            result_x += diff_x;
            result_y += diff_y;
        }

        results.into_iter()
    }).collect();

    // println!("Extensions found: {:?}", externsions);
    println!("Total number of extensions: {}", externsions.len());
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}