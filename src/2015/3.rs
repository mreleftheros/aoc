use std::{collections::HashMap, hash::Hash};

use aoc::get_from_url;

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Eq for Point {}

impl Point {
    fn move_north(&mut self) {
        self.y -= 1;
    }
    fn move_east(&mut self) {
        self.x += 1;
    }
    fn move_south(&mut self) {
        self.y += 1;
    }
    fn move_west(&mut self) {
        self.x -= 1;
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Santa {
    current_pos: Point,
    houses: HashMap<Point, i32>,
}

impl Santa {
    fn new() -> Self {
        let current_pos = Point { x: 0, y: 0 };
        let mut houses = HashMap::new();
        houses.insert(current_pos, 1);

        Self {
            current_pos,
            houses,
        }
    }

    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                self.current_pos.move_north();
            }
            Direction::East => {
                self.current_pos.move_east();
            }
            Direction::South => {
                self.current_pos.move_south();
            }
            Direction::West => {
                self.current_pos.move_west();
            }
        }
        let v = self.houses.entry(self.current_pos).or_insert(0);
        *v += 1;
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = get_from_url("https://adventofcode.com/2015/day/3/input");

    let mut santa = Santa::new();

    contents.chars().for_each(|c| {
        let d = Direction::from(c);
        santa.move_direction(d);
    });

    println!("PART 1: {}", santa.houses.len());

    // part 2
    let mut santa = Santa::new();
    let mut robot = Santa::new();

    contents.chars().enumerate().for_each(|(i, c)| {
        let d = Direction::from(c);

        if i % 2 == 0 {
            santa.move_direction(d);
        } else {
            robot.move_direction(d);
        }
    });

    let mut combined = HashMap::new();

    for (k, v) in santa.houses {
        combined.insert(k, v);
    }

    for (k, v) in robot.houses {
        combined.insert(k, v);
    }

    println!("PART 2: {}", combined.len());
    Ok(())
}
