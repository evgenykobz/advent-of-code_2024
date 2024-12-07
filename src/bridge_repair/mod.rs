use core::panic;
use std::ops::Sub;

/// [Day 7: Bridge Repair](https://adventofcode.com/2024/day/7)
/// What is their total calibration result?
pub fn solve(path: String) {
    let input = std::fs::read_to_string(path).expect("Input file should exist");
    let lines = input.lines().collect::<Vec<&str>>();
    let result: u64 = lines
        .iter()
        .filter_map(|line| {
            line.split_once(": ")
                .and_then(|(sum, numbers)| {
                    sum.parse::<u64>()
                        .and_then(|sum| {
                            let numbers = numbers
                                .split_ascii_whitespace()
                                .map(|number| number.parse::<u32>().expect("Should be a number"))
                                .collect::<Vec<u32>>();
                            Ok((sum, numbers))
                        })
                        .ok()
                })
                .and_then(|(original_sum, numbers)| {
                    let positions = numbers.len().sub(1);
                    match (0..3usize.pow(positions as u32))
                        .map(|i| {
                            (0..positions)
                                .map(|j| match (i / 3usize.pow(j as u32)) % 3 {
                                    0 => "+",
                                    1 => "*",
                                    2 => "||",
                                    _ => unreachable!(),
                                })
                                .collect::<Vec<_>>()
                        })
                        .try_for_each(|operators| {
                            match operators
                                .iter()
                                .enumerate()
                                .fold(
                                    numbers.first().unwrap().clone() as u64,
                                    |sum, (operator_ix, operator)| {
                                        operator_ix
                                            .checked_add(1)
                                            .and_then(|number_ix: usize| numbers.get(number_ix))
                                            .and_then(|number| match operator {
                                                value if value.eq(&"+") => {
                                                    sum.checked_add(*number as u64)
                                                }
                                                value if value.eq(&"*") => {
                                                    sum.checked_mul(*number as u64)
                                                }
                                                value if value.eq(&"||") => {
                                                    let mut sum = sum.to_string();
                                                    sum.push_str(&number.to_string());
                                                    sum.parse::<u64>().ok()
                                                }
                                                _ => unreachable!(),
                                            })
                                            .unwrap_or(sum)
                                    },
                                )
                                .eq(&original_sum)
                            {
                                true => Err(()),
                                false => Ok(()),
                            }
                        }) {
                        Err(_) => Some(original_sum),
                        _ => None,
                    }
                })
        })
        .sum();
    println!("Total calibration result is {result}")
}
