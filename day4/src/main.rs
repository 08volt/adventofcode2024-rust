use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::str::FromStr;


struct Direction {
    dx: i32,
    dy: i32,
}

impl Direction {
    fn new(dx: i32, dy: i32) -> Self {
        Direction { dx, dy }
    }

    fn get_dx(&self) -> i32 {
        self.dx
    }

    fn get_dy(&self) -> i32 {
        self.dy
    }

    fn go(&self, x: i32, y: i32) -> (i32, i32) {
        (x + self.dx, y + self.dy)
    }
}

enum DirectionType {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl DirectionType {
    fn to_direction(&self) -> Direction {
        match self {
            DirectionType::Up => Direction::new(0, -1),
            DirectionType::Down => Direction::new(0, 1),
            DirectionType::Left => Direction::new(-1, 0),
            DirectionType::Right => Direction::new(1, 0),
            DirectionType::UpLeft => Direction::new(-1, -1),
            DirectionType::UpRight => Direction::new(1, -1),
            DirectionType::DownLeft => Direction::new(-1, 1),
            DirectionType::DownRight => Direction::new(1, 1),
        }
    }

    fn go (&self, x: i32, y: i32) -> (i32, i32) {
        self.to_direction().go(x, y)
    }
    
}


struct Matrix {
    map: HashMap<(i32, i32), i32>,
    width: i32,
    height: i32,
}

impl Matrix {
    fn char_to_int(c: char) -> i32 {
        match c {
            'X' => 0,
            'M' => 1,
            'A' => 2,
            'S' => 3,
            _ => panic!("Invalid character: {}", c),
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&i32> {
        self.map.get(&(x, y))
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn explore(&self, x: i32, y: i32, direction: &DirectionType, previous: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }

        let current_value = self.get(x, y).unwrap().clone();

        if  current_value != previous + 1 {
            return false;
        }

        if current_value == 3 {
            return true; // Found the target
        }
        
        let (next_x, next_y) = direction.go(x, y);
        return self.explore(next_x, next_y, direction, current_value)
    }

    fn mas_x(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }

        let current_value = self.get(x, y).unwrap().clone();
        if  current_value != 2 {
            return false;
        }

        let up_right_value = self.get(x + 1, y - 1);
        let down_right_value = self.get(x + 1, y + 1);
        let up_left_value = self.get(x - 1, y - 1);
        let down_left_value = self.get(x - 1, y + 1); 

        let first = match (up_right_value, down_left_value) {
            (Some(&r), Some(&l)) if r != l && ( r == 1 || l == 1) && ( r == 3 || l == 3) => true,
            _ => false,
        };

        let second = match (down_right_value, up_left_value) {
            (Some(&r), Some(&l)) if r != l && ( r == 1 || l == 1) && ( r == 3 || l == 3) => true,
            _ => false,
        };  

        return first && second;

    }
}

impl FromStr for Matrix {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in s.lines().enumerate() {
            height += 1;
            for (x, c) in line.chars().enumerate() {
                map.insert((x as i32, y as i32), Matrix::char_to_int(c));
                width = width.max(x as i32 + 1);
            }
        }

        Ok(Matrix { map, width, height })
    }
}

fn main() {

    const FILE_PATH: &str = "./input.txt";

    let content = match read_full_file(FILE_PATH) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let matrix: Matrix = match content.parse() {
        Ok(matrix) => matrix,
        Err(e) => {
            eprintln!("Error parsing matrix: {}", e);
            return;
        }
    };

    let mut result = 0;

    let mut result2 = 0;

    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            let value = matrix.get(x, y);
            match value {
                Some(v) if *v == 0 => for direction in [
                    DirectionType::Up,
                    DirectionType::Down,
                    DirectionType::Left,
                    DirectionType::Right,
                    DirectionType::UpLeft,
                    DirectionType::UpRight,
                    DirectionType::DownLeft,
                    DirectionType::DownRight,
                ].iter() {
                    if matrix.explore(x, y, direction, -1) {
                        result += 1;
                    }
                },
                _ => {
                    // Do nothing for other values
                },
            }

            if matrix.mas_x(x, y) {
                result2 += 1;
            }
        }
    }

    println!("Total paths found: {}", result);
    println!("Total mas_x found: {}", result2);


}

fn read_full_file<P>(filename: P) -> io::Result<String>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let mut contents = String::new();
    io::BufReader::new(file).read_to_string(&mut contents)?;
    Ok(contents)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}