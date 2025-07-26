use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::{zip, Map};
use std::path::Path;
use std::result;

fn main() {

    const FILE_PATH: &str = "./input.txt";


    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(line) = line {

                let mut numbers = line.split_whitespace();

                let first_number = numbers.next().unwrap().parse::<i32>().unwrap();
                let second_number = numbers.next().unwrap().parse::<i32>().unwrap();

                first_list.push(first_number.clone());
                second_list.push(second_number.clone());
                
            }
        }
    }

    first_list.sort();
    second_list.sort();

    let mut result = 0;

    zip(first_list.clone(), second_list.clone()).for_each(|(a, b)| {
        result += (a - b).abs( ) as i32;
    });

    println!("{}", result);

    let mut second_list_count: HashMap<i32, usize> = HashMap::new();

    for number in second_list.iter() {
        *second_list_count.entry(number.clone()).or_insert(0) += 1;
    }


    let result2 = first_list
        .iter()
        .map(| value| value * *second_list_count.get(value).unwrap_or(&0) as i32 )
        .sum::<i32>();

    println!("{:#?}", second_list_count);

    println!("{}", result2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}