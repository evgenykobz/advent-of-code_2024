use std::ops::Mul;

/// [Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)
/// How many unique locations within the bounds of the map contain an antinode?
pub fn solve(path: String) {
    let input = std::fs::read_to_string(path).expect("Input file should exist");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut locations = std::collections::HashSet::<(usize, usize)>::new();
    lines.iter().enumerate().for_each(|(line_ix, line)| {
        for (symbol_ix, symbol) in line.chars().enumerate() {
            if symbol.to_string().eq(".") {
                continue;
            }
            lines
                .iter()
                .enumerate()
                .for_each(|(pair_line_ix, pair_line)| {
                    for (pair_symbol_ix, _) in pair_line.match_indices(symbol) {
                        let position = (line_ix, symbol_ix);
                        let pair_position = (pair_line_ix, pair_symbol_ix);
                        if position.eq(&pair_position) {
                            continue;
                        }
                        let mut antennas = [position, pair_position];
                        antennas.sort_by(|a, b| a.0.cmp(&b.0));
                        let vertical_distance = antennas[1].0.abs_diff(antennas[0].0);
                        antennas[1]
                            .0
                            .checked_sub(vertical_distance.mul(2))
                            .and_then(|vertical_position| {
                                let horizontal_diff = antennas[0].1.abs_diff(antennas[1].1);
                                let horizontal_position = match antennas[0].1.gt(&antennas[1].1) {
                                    true => antennas[0].1.checked_add(horizontal_diff),
                                    false => antennas[0].1.checked_sub(horizontal_diff),
                                };
                                horizontal_position.and_then(|horizontal_position| {
                                    lines
                                        .get(vertical_position)
                                        .and_then(|line| line.chars().nth(horizontal_position))
                                        .and_then(|_| {
                                            locations
                                                .insert((vertical_position, horizontal_position))
                                                .into()
                                        })
                                })
                            });
                        antennas[0]
                            .0
                            .checked_add(vertical_distance.mul(2))
                            .and_then(|vertical_position| {
                                let horizontal_diff = antennas[0].1.abs_diff(antennas[1].1);
                                let horizontal_position = match antennas[1].1.gt(&antennas[0].1) {
                                    true => antennas[1].1.checked_add(horizontal_diff),
                                    false => antennas[1].1.checked_sub(horizontal_diff),
                                };
                                horizontal_position.and_then(|horizontal_position| {
                                    lines
                                        .get(vertical_position)
                                        .and_then(|line| line.chars().nth(horizontal_position))
                                        .and_then(|_| {
                                            locations
                                                .insert((vertical_position, horizontal_position))
                                                .into()
                                        })
                                })
                            });
                    }
                });
        }
    });
    println!("Part One: unique locations count is {}", locations.len());
}
