use aoc::get_from_url;

fn main() {
    let input = get_from_url("https://adventofcode.com/2024/day/2/input");

    let safe = input
        .lines()
        .filter(|&l| {
            let nums = l
                .split_whitespace()
                .filter_map(|number_string| number_string.parse().ok())
                .collect::<Vec<i32>>();
            are_adjacent_and_ranged(&nums, 1, 3)
        })
        .count();
    println!("PART 1: {}", safe);
}

fn are_adjacent_and_ranged(nums: &[i32], min: i32, max: i32) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let is_increasing = nums[1] > nums[0];

    let r = nums.windows(2).fold(true, |acc, curr| {
        if acc == false {
            return false;
        }
        let is_stepped = if is_increasing {
            curr[1] > curr[0]
        } else {
            curr[0] > curr[1]
        };
        let diff = (curr[1] - curr[0]).abs();
        let is_ranged = diff <= max && diff >= min;
        is_stepped && is_ranged
    });
    r
}
