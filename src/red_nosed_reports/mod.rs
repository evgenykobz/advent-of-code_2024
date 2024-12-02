/// [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)
/// How many reports are safe?
pub fn solve(path: String) {
    let input = std::fs::read_to_string(path).expect("Input file should exist");
    let mapped_levels = input
        .lines()
        .map(|line| {
            check_levels(
                line.split_ascii_whitespace()
                    .map(|token| token.parse::<u32>().expect("Token should be a number"))
                    .collect(),
            )
        })
        .collect::<Vec<Result<(), Vec<u32>>>>();
    let safe_levels = mapped_levels.iter().filter(|line| line.is_ok()).count();
    println!("Part 1: Safe levels count is {safe_levels}");
    let fixed_levels = mapped_levels
        .iter()
        .filter(|line| line.is_err())
        .map(|line| {
            let levels = line
                .clone()
                .err()
                .expect("Result should be an error at this point");
            let fixed_levels = fix_level(&levels);
            fixed_levels
        })
        .filter(|levels| check_levels(levels.clone()).is_ok())
        .count();
    println!(
        "Part 2: {} levels fixed by the Problem Dampener. Safe levels count is {}",
        fixed_levels,
        fixed_levels + safe_levels
    )
}
fn check_levels(levels: Vec<u32>) -> Result<(), Vec<u32>> {
    let mut two_levels = levels.iter().take(2);
    let initial_value = two_levels
        .next()
        .expect("Level list should have at least 2 elements");
    match two_levels
        .next()
        .expect("Level list should have at least 2 elements")
    {
        second_value if second_value > initial_value => Ok(Direction::Increasing),
        second_value if second_value < initial_value => Ok(Direction::Decreasing),
        _ => Err(()),
    }
    .and_then(|direction| {
        let mut iter = levels.iter();
        iter.next();
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
    .map(|_| ())
    .map_err(|_| levels)
}
fn fix_level(levels: &Vec<u32>) -> Vec<u32> {
    let mut output = vec![*levels.first().expect("Level list should be non-empty")];
    let direction = match levels
        .last()
        .expect("Level list should be non-empty")
        .gt(output.first().unwrap())
    {
        true => Direction::Increasing,
        false => Direction::Decreasing,
    };
    let mut error_found = false;
    for ix in 1..levels.len() {
        let level = *levels.get(ix).unwrap();
        let last_level = *output.last().unwrap();
        if error_found {
            output.push(level);
            continue;
        }
        match direction {
            Direction::Increasing => {
                let diff = level as i32 - last_level as i32;
                if ix == 1 {
                    let next_level = *levels.get(ix + 1).unwrap();
                    if diff < 1 || diff > 3 {
                        let next_sub_last = next_level as i32 - last_level as i32;
                        if next_sub_last < 1 || next_sub_last > 3 && !error_found {
                            output.pop();
                            output.push(level);
                            error_found = true;
                        } else if !error_found {
                            error_found = true;
                        }
                    } else {
                        output.push(level);
                    }
                } else if ix == levels.len() - 1 {
                    if diff < 1 || diff > 3 && !error_found {
                        error_found = true;
                    } else {
                        output.push(level);
                    }
                } else if diff < 1 || diff > 3 && !error_found {
                    error_found = true;
                } else {
                    output.push(level);
                }
            }
            Direction::Decreasing => {
                let diff = last_level as i32 - level as i32;
                if ix == 1 {
                    let next_level = *levels.get(ix + 1).unwrap();
                    if diff < 1 || diff > 3 {
                        let last_sub_next = last_level as i32 - next_level as i32;
                        if last_sub_next < 1 || last_sub_next > 3 && !error_found {
                            output.pop();
                            output.push(level);
                            error_found = true;
                        } else if !error_found {
                            error_found = true;
                        }
                    } else {
                        output.push(level);
                    }
                } else if ix == levels.len() - 1 {
                    if diff < 1 || diff > 3 && !error_found {
                        error_found = true;
                    } else {
                        output.push(level);
                    }
                } else if diff < 1 || diff > 3 && !error_found {
                    error_found = true;
                } else {
                    output.push(level);
                }
            }
        };
    }
    output
}
#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}
