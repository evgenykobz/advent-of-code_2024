/// [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)
/// How many reports are safe?
pub fn solve(path: String) {
    let safe_levels = std::fs::read_to_string(path)
        .expect("Input file should exist")
        .lines()
        .map(|line| {
            let levels = line
                .split_ascii_whitespace()
                .map(|token| token.parse::<u32>().expect("Token should be a number"))
                .collect::<Vec<u32>>();
            let mut two_levels = levels.iter().take(2);
            let initial_value = two_levels
                .next()
                .expect("Level list should have at least 2 elements");
            match two_levels
                .next()
                .expect("Level list should have at least 2 elements")
            {
                value if value > initial_value => Ok(Direction::Increasing),
                value if value < initial_value => Ok(Direction::Decreasing),
                _ => Err(()),
            }
            .and_then(|direction| {
                let mut iter = levels.iter();
                let initial_value = iter
                    .next()
                    .expect("Level list should have at least 1 element");
                iter.try_fold(*initial_value, |acc, x| {
                    let diff = match direction {
                        Direction::Increasing => acc.lt(x).then_some(acc.abs_diff(*x)),
                        Direction::Decreasing => acc.gt(x).then_some(acc.abs_diff(*x)),
                    };
                    match diff.map(|diff| diff.le(&3)) {
                        Some(true) => Ok(*x),
                        _ => Err(()),
                    }
                })
            })
        })
        .filter(|line| line.is_ok())
        .count();
    println!("Part 1: Safe levels count is {safe_levels}");
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}
