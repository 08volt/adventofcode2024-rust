use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;


struct Map {
    map: HashMap<(i32, i32), bool>, // true for wall, false for empty
    width: i32,
    height: i32,
}

impl Map {
    fn visit(&self, guard: Guard) -> (bool, HashSet<((i32, i32), Direction)>) {
        let mut guard = guard.clone();
        let mut current_position = guard.position;
        let mut current_direction = guard.direction.clone();
        let mut visited_positions = HashSet::new();
        while current_position.0 >= 0 && current_position.0 < self.width &&
              current_position.1 >= 0 && current_position.1 < self.height {
            visited_positions.insert((current_position, current_direction.clone()));
            let mut next_position = current_direction.move_guard(&guard);
            // If the next position is a wall, turn right
            while *self.map.get(&next_position).unwrap_or(&false) {
                current_direction = current_direction.turn_right();
                next_position = current_direction.move_guard(&guard);
            }
            current_position = next_position;
            guard.position = current_position;
            guard.direction = current_direction.clone();
            // If we have already visited this position with this direction, we are in a loop
            if visited_positions.contains(&(current_position, current_direction.clone())) {
                return (true, visited_positions);
            }
        }
        (false, visited_positions)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {

    fn cast_char(s: char, x: i32, y: i32) -> Result<Self, ()> {
        match s {
            '^' => Ok(Guard {
                position: (x, y),
                direction: Direction::Up,
            }),
            'v' => Ok(Guard {
                position: (x, y),
                direction: Direction::Down,
            }),
            '<' => Ok(Guard {
                position: (x, y),
                direction: Direction::Left,
            }),
            '>' => Ok(Guard {
                position: (x, y),
                direction: Direction::Right,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_guard(&self, guard: &Guard) -> (i32, i32) {
        match self {
            Direction::Up => (guard.position.0 - 1, guard.position.1),
            Direction::Down => (guard.position.0 + 1, guard.position.1),
            Direction::Left => (guard.position.0, guard.position.1 - 1),
            Direction::Right => (guard.position.0, guard.position.1 + 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn main() {

    const FILE_PATH: &str = "./input.txt";

    let mut map: HashMap<(i32, i32), bool> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    let mut guard_char = 'o';
    let mut guard_position = (0, 0);

    // Open the text file
    if let Ok(lines) = read_lines(FILE_PATH) {
        lines.enumerate().for_each(|(x,line)| {
            if let Ok(line) = line {
                for (y, char) in line.chars().enumerate() {
                    map.insert((x as i32, y as i32), char == '#');
                    width = width.max(y as i32 + 1);
                    height = height.max(x as i32 + 1);
                    if char != '.' && char != '#' {
                        guard_char = char;
                        guard_position = (x as i32, y as i32);
                    }
                }
            }
        });
    }
    let guard = Guard::cast_char(guard_char, guard_position.0, guard_position.1)
        .expect("Invalid guard character");
    // Part One: Count distinct positions visited
    let (_, steps) = Map {
        map: map.clone(),
        width,
        height,
    }.visit(guard.clone());

    println!("Part One: Distinct positions visited: {}", steps.iter().map(|(pos, _)| pos).collect::<HashSet<_>>().len());

    // Part Two: Count positions where an obstruction causes a loop
    let mut loop_count = 0;
    let unique_positions: HashSet<(i32, i32)> = steps.iter()
        .map(|(pos, _)| *pos)
        .collect();
    for (x, y) in unique_positions.iter() {
        let pos = (*x, *y);
        // Skip the guard's starting position and existing walls
        if pos == guard_position || *map.get(&pos).unwrap_or(&false) {
            continue;
        }
        // Place an obstruction at (x, y)
        let mut new_map = map.clone();
        new_map.insert(pos, true);
        let m = Map {
            map: new_map,
            width,
            height,
        };
        // Check if placing an obstruction causes a loop
        let (looped, _) = m.visit(guard.clone());
        if looped {
            loop_count += 1;
        }
    }

    println!("Part Two: Number of positions causing a loop: {}", loop_count);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}