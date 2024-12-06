use crate::util;

pub fn run() {
    let input = util::get_input_from_url("https://adventofcode.com/2024/day/2/input");
    let total = input.lines().count();
    dbg!(total);

    let safe = input
        .lines()
        .filter(|&l| {
            let nums = l
                .split_whitespace()
                .filter_map(|number_string| number_string.parse().ok())
                .collect::<Vec<i32>>();
            get_unsafe_indices(&nums, 1, 3).len() == 0
        })
        .count();
    println!("PART 1: {}", safe);

    // part 2
    let safe = input
        .lines()
        .filter(|&l| {
            let nums = l
                .split_whitespace()
                .filter_map(|number_string| number_string.parse().ok())
                .collect::<Vec<i32>>();
            let unsafe_indices = get_unsafe_indices(&nums, 1, 3);
            if unsafe_indices.len() == 0 {
                return true;
            } else {
                let r = unsafe_indices.iter().fold(false, |mut acc, curr| {
                    if acc {
                        return true;
                    }
                    let nums_filtered = nums
                        .iter()
                        .enumerate()
                        .filter(|(idx, &x)| idx != curr)
                        .map(|(idx, &x)| x)
                        .collect::<Vec<_>>();
                    let u = get_unsafe_indices(&nums_filtered, 1, 3);
                    if u.len() == 0 {
                        acc = true;
                    }
                    acc
                });
                r
            }
        })
        .count();
    println!("PART 2: {}", safe);
}

fn get_unsafe_indices(nums: &[i32], min: i32, max: i32) -> Vec<usize> {
    let unsafe_indices_vec = Vec::new();
    if nums.len() < 2 {
        return unsafe_indices_vec;
    }

    let is_increasing = nums[1] > nums[0];

    let r = nums
        .windows(2)
        .enumerate()
        .fold(unsafe_indices_vec, |mut acc, curr| {
            let is_stepped = if is_increasing {
                curr.1[1] > curr.1[0]
            } else {
                curr.1[0] > curr.1[1]
            };
            let diff = (curr.1[1] - curr.1[0]).abs();
            let is_ranged = diff <= max && diff >= min;
            if !is_stepped || !is_ranged {
                acc.push(curr.0 + 1);
            }
            acc
        });
    r
}
