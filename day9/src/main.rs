use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;



fn main() {

    let start = Instant::now();

    const FILE_PATH: &str = "./input.txt";

    // Open the text file
    let line = read_line(FILE_PATH).expect("Failed to read line from file");

    let result = optimize_space_part1(&line);

    let checksum = result.iter().enumerate().fold(0, |acc, (i, &val)| acc + i as u64 * val);

    println!("Checksum: {}", checksum);

    let duration = start.elapsed();
    
    println!("Time elapsed: {:?}", duration);

    let start = Instant::now();
    let result = optimize_space_part2(&line);
    println!("Checksum: {}", result);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);


}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_line<P>(filename: P) -> Option<String>
where P: AsRef<Path>, {
    let file = File::open(filename).ok()?;
    let reader = io::BufReader::new(file);
    reader.lines().next().unwrap_or(Ok("".into())).ok()
}


fn optimize_space_part1(input: &str) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let input_chars = input.chars().collect::<Vec<char>>();

    let mut i_r = if (input.len() - 1) % 2 == 0 { input.len() - 1 } else { input.len() - 2};
    let mut i_l = 0;
    let mut remaining = input_chars[i_r].to_digit(10).unwrap();


    while i_l < i_r {
        let filling_spaces = input_chars[i_l].to_digit(10).unwrap();

        match i_l % 2 == 0 {
            true=> {
                for _ in 0..filling_spaces {
                    result.push((i_l / 2) as u64);
                }
            }
            false => {
                
                for _ in 0..filling_spaces {
                    while remaining == 0 {
                        
                        i_r -= 2;
                        if i_r <= i_l {
                            break
                        }
                        remaining = input_chars[i_r].to_digit(10).unwrap();
                    }

                    if i_r <= i_l {
                        break
                    }
                    
                    result.push((i_r / 2) as u64);
                    remaining -= 1;
                }
            }
        }
        i_l += 1;
    }

    if remaining > 0 {
        for _ in 0..remaining {
            result.push((i_r / 2) as u64);
        }
    }

    result
}

fn optimize_space_part2(input: &str) -> u64 {
    let input_chars = input.chars().collect::<Vec<char>>();

    // first one with enough space up to current starting position -> reduce spece and move starting position if space 0 -> pop

    // vector of ID with starting position and lenght, pop and create new vector


    let mut free_spaces: Vec<(usize, u64)> = Vec::new();
    let mut file_lenght_position: Vec<(usize, u64, usize)> = Vec::new();
    let mut current_pos: usize = 0;

    input_chars.iter().enumerate().for_each(|(i, c)| {
        let number = c.to_digit(10).unwrap();

        if number > 0 {
            if i % 2 == 0 {
                // i // 2 is the ID and c is the lenght
                let id = i / 2 ;
                file_lenght_position.push((id, number.into(), current_pos));
            } else {
                free_spaces.push((current_pos, number.into()));
            }
        }
        current_pos += number as usize
    });

    let mut final_position: Vec<(usize, u64, usize)> = Vec::new();

    file_lenght_position.iter().rev().for_each(|(id, lenght, start_pos)| {
        // println!("Final position: {:?}", final_position);
        // println!("Free spaces: {:?}", free_spaces);
        // println!("Considering id: {} (lenght {} and starting position {})", id, lenght, start_pos);

        let mut moving: Option<(usize, usize, u64)> = None;

        for (i, (pos, size)) in free_spaces.iter().enumerate() {
            if pos >= start_pos {
                break
            }
            if size >= lenght {
                moving = Some((i.clone(), pos.clone(), size.clone()));
                break
            }
        }

        let final_pos = match moving {
            Some((i, pos, size)) => {               
                // move file
                // println!("moving to pos {}", pos);
                free_spaces[i] = (pos + *lenght as usize, size - *lenght);
                pos
            }
            None => {
                // println!("not moving");
                start_pos.clone()
            }
        };

        final_position.push((id.clone(), lenght.clone(), final_pos.clone()))
    });

    // println!("Final position: {:?}", final_position);


    final_position.iter().fold(0, |acc, ( id, lenght, pos)| {
        
        let mut sum = 0;
        for i in *pos..(*pos + *lenght as usize) {
            sum += (i as u64) * (*id as u64);
        }

        acc + sum

    })

}