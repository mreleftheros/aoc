use aoc::get_from_url;

fn main() {
    let input = get_from_url("https://adventofcode.com/2024/day/1/input");

    let mut list1 = vec![];
    let mut list2 = vec![];

    input.lines().for_each(|l| {
        let r = l
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<i32>>();
        list1.push(r[0]);
        list2.push(r[1]);
    });

    list1.sort();
    list2.sort();

    let r = list1
        .iter()
        .zip(list2.iter())
        .map(|(first, second)| (first - second).abs())
        .sum::<i32>();
    println!("PART 1: {}", r);
}
