use crate::util;

pub fn run() {
    let input = util::get_input_from_url("https://adventofcode.com/2015/day/1/input");
    // part 1
    let r = input
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        })
        .reduce(|acc, curr| acc + curr)
        .unwrap_or(0);
    println!("PART 1: {}", r);
}
