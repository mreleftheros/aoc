use crate::util::get_input_from_url;

pub fn run() {
    let input = get_input_from_url("https://adventofcode.com/2024/day/3/input");

    // part 1
    let res = input
        .split("mul(")
        .filter_map(|l| {
            let idx = l.chars().position(|c| c == ')');
            match idx {
                Some(idx) => {
                    let line = l[..idx].to_owned();
                    let parts = line.split(",").collect::<Vec<&str>>();
                    if parts.len() != 2 {
                        return None;
                    }
                    if parts[0].chars().count() < 1 || parts[0].chars().count() > 3 {
                        return None;
                    }
                    if parts[1].chars().count() < 1 || parts[1].chars().count() > 3 {
                        return None;
                    }
                    let p1 = parts[0].parse::<i64>().unwrap_or(0);
                    let p2 = parts[1].parse::<i64>().unwrap_or(0);
                    Some(p1 * p2)
                }
                None => None,
            }
        })
        .reduce(|acc, curr| acc + curr)
        .unwrap_or(0);
    println!("PART 1: {}", res);

    // part 2
    let mut all_indices = input
        .match_indices("do()")
        .chain(input.match_indices("don't()"))
        .collect::<Vec<_>>();

    all_indices.sort_by_key(|k| k.0);
    let enabled_total =
        all_indices
            .windows(2)
            .filter(|&w| w[0].1 == "do()")
            .fold(0, |acc, curr| {
                let total = input[curr[0].0..curr[1].0]
                    .split("mul(")
                    .filter_map(|l| {
                        let idx = l.chars().position(|c| c == ')');
                        match idx {
                            Some(idx) => {
                                let line = l[..idx].to_owned();
                                let parts = line.split(",").collect::<Vec<&str>>();
                                if parts.len() != 2 {
                                    return None;
                                }
                                if parts[0].chars().count() < 1 || parts[0].chars().count() > 3 {
                                    return None;
                                }
                                if parts[1].chars().count() < 1 || parts[1].chars().count() > 3 {
                                    return None;
                                }
                                let p1 = parts[0].parse::<i64>().unwrap_or(0);
                                let p2 = parts[1].parse::<i64>().unwrap_or(0);
                                Some(p1 * p2)
                            }
                            None => None,
                        }
                    })
                    .reduce(|acc, curr| acc + curr)
                    .unwrap_or(0);
                acc + total
            });

    println!("PART 2: {}", enabled_total);
}
