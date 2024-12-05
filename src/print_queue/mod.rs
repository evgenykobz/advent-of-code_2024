use std::ops::{Add, Sub};

/// [Day 5: Print Queue](https://adventofcode.com/2024/day/5)
/// What do you get if you add up the middle page number from those correctly-ordered updates?
pub fn solve(path: String) {
    let input = std::fs::read_to_string(path).expect("Input file should exist");
    let lines = input.lines().collect::<Vec<&str>>();
    let (rules, updates) = lines
        .iter()
        .position(|line| line.is_empty())
        .and_then(|divider_ix| lines.split_at(divider_ix.add(1)).into())
        .expect("Input should have an empty line divider");
    let result: u32 = updates
        .iter()
        .filter_map(|update| {
            let update = update
                .split(",")
                .map(|value| value.parse::<u32>().expect("Value should be a number"))
                .collect::<Vec<u32>>();
            update
                .iter()
                .enumerate()
                .rev()
                .try_for_each(|(ix, value)| {
                    update
                        .get(ix.add(1)..update.len())
                        .or(Some(&[]))
                        .and_then(|numbers| {
                            numbers
                                .iter()
                                .try_for_each(|number| {
                                    rules
                                        .iter()
                                        .find(|entry| {
                                            entry.to_string().eq(&format!("{value}|{number}"))
                                        })
                                        .map(|_| ())
                                        .ok_or(())
                                })
                                .ok()
                        })
                        .ok_or(())
                })
                .ok()
                .and_then(|_| update.get(update.len().div_ceil(2).sub(1)).copied())
        })
        .sum();
    println!("Part One: sum is {result}");
    let result: u32 = updates
        .iter()
        .filter_map(|update| {
            let mut update = update
                .split(",")
                .map(|value| value.parse::<u32>().expect("Value should be a number"))
                .collect::<Vec<u32>>();
            match update.iter().enumerate().rev().try_for_each(|(ix, value)| {
                update
                    .get(ix.add(1)..update.len())
                    .or(Some(&[]))
                    .and_then(|numbers| {
                        numbers
                            .iter()
                            .try_for_each(|number| {
                                rules
                                    .iter()
                                    .find(|entry| {
                                        entry.to_string().eq(&format!("{value}|{number}"))
                                    })
                                    .map(|_| ())
                                    .ok_or(())
                            })
                            .ok()
                    })
            }) {
                None => {
                    update.sort_by(|a, b| {
                        rules
                            .iter()
                            .find(|entry| entry.to_string().eq(&format!("{a}|{b}")))
                            .map(|_| std::cmp::Ordering::Less)
                            .unwrap_or(std::cmp::Ordering::Greater)
                    });
                    update.get(update.len().div_ceil(2).sub(1)).copied()
                }
                _ => None,
            }
        })
        .sum();
    println!("Part One: reordered updates sum is {result}");
}
