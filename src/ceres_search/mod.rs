use std::ops::Add;

/// [Day 4: Ceres Search](https://adventofcode.com/2024/day/4)
/// How many times does XMAS appear?
pub fn solve(path: String) {
    let input = std::fs::read_to_string(path).expect("Input file should exist");
    let lines = input.lines().collect::<Vec<&str>>();
    let result = lines.iter().enumerate().fold(0, |total, (line_ix, line)| {
        let line_total = line.match_indices("X").fold(0, |line_total, (x_ix, _)| {
            let horizontal_forwards = line
                .get(x_ix + 1..x_ix + 4)
                .and_then(|substring| substring.eq("MAS").then_some(1))
                .unwrap_or(0);
            let horizontal_backwards = x_ix
                .checked_sub(3)
                .and_then(|start_ix| line.get(start_ix..x_ix))
                .and_then(|substring| substring.eq("SAM").then_some(1))
                .unwrap_or(0);
            let vertical_upwards = line_ix
                .checked_sub(3)
                .and_then(|end_line_ix| {
                    [end_line_ix + 2, end_line_ix + 1, end_line_ix]
                        .iter()
                        .try_fold(String::new(), |mut acc, line_ix| {
                            lines
                                .get(*line_ix)
                                .and_then(|line| line.chars().nth(x_ix))
                                .and_then(|letter| {
                                    acc.push_str(letter.to_string().as_str());
                                    acc.into()
                                })
                        })
                })
                .and_then(|word| word.eq("MAS").then_some(1))
                .unwrap_or(0);
            let vertical_downwards = line_ix
                .checked_add(3)
                .and_then(|end_line_ix| {
                    [end_line_ix - 2, end_line_ix - 1, end_line_ix]
                        .iter()
                        .try_fold(String::new(), |mut acc, line_ix| {
                            lines
                                .get(*line_ix)
                                .and_then(|line| line.chars().nth(x_ix))
                                .and_then(|letter| {
                                    acc.push_str(letter.to_string().as_str());
                                    acc.into()
                                })
                        })
                })
                .and_then(|word| word.eq("MAS").then_some(1))
                .unwrap_or(0);
            let diagonal_top_left = line_ix
                .checked_sub(3)
                .and_then(|end_line_ix| {
                    [end_line_ix + 2, end_line_ix + 1, end_line_ix]
                        .iter()
                        .enumerate()
                        .try_fold(String::new(), |mut acc, (letter_ix, line_ix)| {
                            x_ix.checked_sub(letter_ix.add(1))
                                .and_then(|letter_ix| {
                                    lines
                                        .get(*line_ix)
                                        .and_then(|line| line.chars().nth(letter_ix))
                                })
                                .and_then(|letter| {
                                    acc.push_str(letter.to_string().as_str());
                                    acc.into()
                                })
                        })
                })
                .and_then(|word| word.eq("MAS").then_some(1))
                .unwrap_or(0);
            let diagonal_top_right = line_ix
                .checked_sub(3)
                .and_then(|end_line_ix| {
                    [end_line_ix + 2, end_line_ix + 1, end_line_ix]
                        .iter()
                        .enumerate()
                        .try_fold(String::new(), |mut acc, (letter_ix, line_ix)| {
                            x_ix.checked_add(letter_ix.add(1))
                                .and_then(|letter_ix| {
                                    lines
                                        .get(*line_ix)
                                        .and_then(|line| line.chars().nth(letter_ix))
                                })
                                .and_then(|letter| {
                                    acc.push_str(letter.to_string().as_str());
                                    acc.into()
                                })
                        })
                })
                .and_then(|word| word.eq("MAS").then_some(1))
                .unwrap_or(0);
            let diagonal_bottom_left = line_ix
                .checked_add(3)
                .and_then(|end_line_ix| {
                    [end_line_ix - 2, end_line_ix - 1, end_line_ix]
                        .iter()
                        .enumerate()
                        .try_fold(String::new(), |mut acc, (letter_ix, line_ix)| {
                            x_ix.checked_sub(letter_ix.add(1))
                                .and_then(|letter_ix| {
                                    lines
                                        .get(*line_ix)
                                        .and_then(|line| line.chars().nth(letter_ix))
                                })
                                .and_then(|letter| {
                                    acc.push_str(letter.to_string().as_str());
                                    acc.into()
                                })
                        })
                })
                .and_then(|word| word.eq("MAS").then_some(1))
                .unwrap_or(0);
            let diagonal_bottom_right = line_ix
                .checked_add(3)
                .and_then(|end_line_ix| {
                    [end_line_ix - 2, end_line_ix - 1, end_line_ix]
                        .iter()
                        .enumerate()
                        .try_fold(String::new(), |mut acc, (letter_ix, line_ix)| {
                            x_ix.checked_add(letter_ix.add(1))
                                .and_then(|letter_ix| {
                                    lines
                                        .get(*line_ix)
                                        .and_then(|line| line.chars().nth(letter_ix))
                                })
                                .and_then(|letter| {
                                    acc.push_str(letter.to_string().as_str());
                                    acc.into()
                                })
                        })
                })
                .and_then(|word| word.eq("MAS").then_some(1))
                .unwrap_or(0);
            line_total
                + horizontal_forwards
                + horizontal_backwards
                + vertical_upwards
                + vertical_downwards
                + diagonal_top_left
                + diagonal_top_right
                + diagonal_bottom_left
                + diagonal_bottom_right
        });
        total + line_total
    });
    println!("Part One: XMAS appears {result} times");
    let result = lines.iter().enumerate().fold(0, |total, (line_ix, line)| {
        let line_total = line.match_indices("A").fold(0, |line_total, (a_ix, _)| {
            let iter_result = line_ix
                .checked_sub(1)
                .and_then(|line_ix| lines.get(line_ix))
                .and_then(|line| {
                    a_ix.checked_sub(1)
                        .and_then(|letter_ix| line.chars().nth(letter_ix))
                        .and_then(|letter| letter.to_string().into())
                        .and_then(|letter| {
                            letter
                                .eq("M")
                                .then_some("M")
                                .or_else(|| letter.eq("S").then_some("S"))
                        })
                        .and_then(|top_left_letter| {
                            line_ix
                                .checked_add(1)
                                .and_then(|line_ix| lines.get(line_ix))
                                .and_then(|line| {
                                    a_ix.checked_add(1)
                                        .and_then(|letter_ix| line.chars().nth(letter_ix))
                                        .and_then(|letter| letter.to_string().into())
                                        .and_then(|letter| {
                                            if top_left_letter.eq("M") && letter.eq("S") {
                                                Some(())
                                            } else if top_left_letter.eq("S") && letter.eq("M") {
                                                Some(())
                                            } else {
                                                None
                                            }
                                        })
                                })
                        })
                        .and_then(|_| line_ix.checked_sub(1))
                        .and_then(|line_ix| lines.get(line_ix))
                        .and_then(|line| {
                            a_ix.checked_add(1)
                                .and_then(|letter_ix| line.chars().nth(letter_ix))
                                .and_then(|letter| letter.to_string().into())
                                .and_then(|letter| {
                                    letter
                                        .eq("M")
                                        .then_some("M")
                                        .or_else(|| letter.eq("S").then_some("S"))
                                })
                        })
                        .and_then(|top_right_letter| {
                            line_ix
                                .checked_add(1)
                                .and_then(|line_ix| lines.get(line_ix))
                                .and_then(|line| {
                                    a_ix.checked_sub(1)
                                        .and_then(|letter_ix| line.chars().nth(letter_ix))
                                        .and_then(|letter| letter.to_string().into())
                                        .and_then(|letter| {
                                            if top_right_letter.eq("M") && letter.eq("S") {
                                                Some(())
                                            } else if top_right_letter.eq("S") && letter.eq("M") {
                                                Some(())
                                            } else {
                                                None
                                            }
                                        })
                                })
                        })
                })
                .map(|_| 1)
                .unwrap_or(0);
            line_total + iter_result
        });
        total + line_total
    });
    println!("Part Two: X-MAS appears {result} times");
}
