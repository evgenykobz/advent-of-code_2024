use std::ops::{Add, Mul, Not};

/// [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)
/// What do you get if you add up all of the results of the multiplications?
pub fn solve(path: String) {
    let input = std::fs::read_to_string(path).expect("Input file should exist");
    let mul_result = input.lines().fold(0, |total, line| {
        line.match_indices("mul(")
            .fold(0, |line_total, (ix, _)| {
                let start_ix = ix + 4;
                line.get(start_ix..line.len())
                    .and_then(|substring| substring.find(")"))
                    .and_then(|end_ix| {
                        line.get(start_ix..start_ix + end_ix)
                            .and_then(|token| token.contains("mul(").not().then_some(token))
                    })
                    .and_then(|token| token.splitn(2, ",").collect::<Vec<&str>>().into())
                    .and_then(|slices| slices.len().eq(&2).then_some(slices))
                    .and_then(|slices| {
                        slices
                            .first()
                            .and_then(|first_number| first_number.parse::<i32>().ok())
                            .and_then(|first_number| {
                                slices
                                    .last()
                                    .and_then(|last_number| last_number.parse::<i32>().ok())
                                    .and_then(|last_number| last_number.mul(first_number).into())
                            })
                    })
                    .unwrap_or(0)
                    .add(line_total)
            })
            .add(total)
    });
    println!("Part 1: Multiplication result is {mul_result}");
    let line = input.lines().collect::<Vec<&str>>().join("");
    let mul_result = line.match_indices("mul(").fold(0, |line_total, (ix, _)| {
        let start_ix = ix + 4;
        let increment = line
            .get(start_ix..line.len())
            .and_then(|substring| substring.find(")"))
            .and_then(|end_ix| {
                line.get(start_ix..start_ix + end_ix)
                    .and_then(|token| token.contains("mul(").not().then_some(token))
            })
            .and_then(|token| token.splitn(2, ",").collect::<Vec<&str>>().into())
            .and_then(|slices| slices.len().eq(&2).then_some(slices))
            .and_then(|slices| {
                slices
                    .first()
                    .and_then(|first_number| first_number.parse::<i32>().ok())
                    .and_then(|first_number| {
                        slices
                            .last()
                            .and_then(|last_number| last_number.parse::<i32>().ok())
                            .and_then(|last_number| last_number.mul(first_number).into())
                    })
            })
            .unwrap_or(0);
        line.get(..start_ix)
            .and_then(|substring| substring.rfind("don't()"))
            .and_then(|dont_ix| {
                line.get(dont_ix..start_ix)
                    .and_then(|substring| substring.rfind("do()"))
                    .and(line_total.add(increment).into())
                    .or(line_total.into())
            })
            .unwrap_or(line_total.add(increment))
    });
    println!("Part 2: Multiplication result is {mul_result}");
}
