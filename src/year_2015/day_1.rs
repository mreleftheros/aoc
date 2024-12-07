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

    // part 2
    let r = input
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None,
        })
        .collect::<Vec<i32>>();
    let mut total = 0;
    for (i, item) in r.into_iter().enumerate() {
        total += item;
        if total == -1 {
            total = (i + 1) as i32;
            break;
        }
    }
    println!("PART 2: {}", total);
}
