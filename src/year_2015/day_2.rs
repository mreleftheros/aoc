use crate::util;

#[derive(Debug)]
struct Present {
    length: i64,
    width: i64,
    height: i64,
}

impl Present {
    fn new(length: i64, width: i64, height: i64) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    fn get_surface_area(&self) -> i64 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
    }

    fn get_smallest_area(&self) -> i64 {
        let mut smallest = self.length * self.width;
        if self.width * self.height < smallest {
            smallest = self.width * self.height;
        }
        if self.height * self.length < smallest {
            smallest = self.height * self.length;
        }
        smallest
    }

    fn get_wrapping_paper(&self) -> i64 {
        self.get_surface_area() + self.get_smallest_area()
    }

    fn get_ribbon(&self) -> i64 {
        let mut dims = [self.length, self.width, self.height];
        dims.sort();
        let ribbon = dims.into_iter().take(2).fold(0, |mut acc, curr| {
            acc += curr * 2;
            acc
        });
        ribbon + (self.length * self.width * self.height)
    }
}

pub fn run() {
    let input = util::get_input_from_url("https://adventofcode.com/2015/day/2/input");

    // part 1
    let r = input
        .trim()
        .lines()
        .filter_map(|l| {
            let dims = l.split("x").collect::<Vec<_>>();
            if dims.len() != 3 {
                return None;
            }

            let length = match dims[0].parse::<i64>().ok() {
                Some(v) => v,
                None => return None,
            };

            let width = match dims[1].parse::<i64>().ok() {
                Some(v) => v,
                None => return None,
            };
            let height = match dims[2].parse::<i64>().ok() {
                Some(v) => v,
                None => return None,
            };

            Some(Present::new(length, width, height))
        })
        .map(|p| p.get_wrapping_paper())
        .sum::<i64>();
    println!("PART 1: {}", r);

    // part 2
    let r = input
        .trim()
        .lines()
        .filter_map(|l| {
            let dims = l.split("x").collect::<Vec<_>>();
            if dims.len() != 3 {
                return None;
            }

            let length = match dims[0].parse::<i64>().ok() {
                Some(v) => v,
                None => return None,
            };

            let width = match dims[1].parse::<i64>().ok() {
                Some(v) => v,
                None => return None,
            };
            let height = match dims[2].parse::<i64>().ok() {
                Some(v) => v,
                None => return None,
            };

            Some(Present::new(length, width, height))
        })
        .map(|p| p.get_ribbon())
        .sum::<i64>();
    println!("PART 2: {}", r);
}
