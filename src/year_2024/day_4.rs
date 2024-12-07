use crate::util;

pub fn run() {
    let input = util::get_input_from_url("https://adventofcode.com/2024/day/4/input");
    // part 1
    let lines = input.lines().collect::<Vec<_>>();
    dbg!(&lines, &lines[0].len(), &lines[1].len());
}
