use std::ops::{Add, Not};

/// [Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)
/// How many distinct positions will the guard visit before leaving the mapped area?
pub fn solve(path: String) {
    let input = std::fs::read_to_string(path).expect("Input file should exist");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut guard = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains("^"))
        .and_then(|(line_ix, line)| {
            line.find("^")
                .and_then(|position_ix| Some(Guard::new((line_ix, position_ix))))
        })
        .expect("Guard should have a starting position");
    guard.move_position(&lines);
    println!(
        "Part One: guard has visited {} positions",
        guard.positions.len()
    );
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct Guard {
    position: (usize, usize),
    direction: Direction,
    pub positions: Vec<(usize, usize)>,
}
impl Guard {
    pub fn new(position: (usize, usize)) -> Self {
        let mut positions = Vec::new();
        positions.push(position.clone());
        Self {
            position,
            direction: Direction::Up,
            positions,
        }
    }
    pub fn move_position(&mut self, map: &Vec<&str>) {
        let new_position = match self.direction {
            Direction::Up => map.get(0..self.position.0).and_then(|lines| {
                match lines
                    .iter()
                    .enumerate()
                    .rev()
                    .try_for_each(|(line_ix, line)| {
                        line.chars()
                            .nth(self.position.1)
                            .and_then(|char| {
                                char.to_string().ne("#").then(|| {
                                    let position = (line_ix, self.position.1);
                                    self.positions.contains(&position).not().then(|| {
                                        self.positions.push(position);
                                    });
                                    ()
                                })
                            })
                            .ok_or(line_ix.add(1))
                    }) {
                    Ok(_) => None,
                    Err(line_ix) => {
                        self.direction = Direction::Right;
                        Some((line_ix, self.position.1))
                    }
                }
            }),
            Direction::Down => map
                .get(self.position.0.add(1)..map.len())
                .and_then(|lines| {
                    match lines.iter().enumerate().try_for_each(|(line_ix, line)| {
                        line.chars()
                            .nth(self.position.1)
                            .and_then(|char| {
                                char.to_string().ne("#").then(|| {
                                    let position =
                                        (self.position.0.add(line_ix).add(1), self.position.1);
                                    self.positions.contains(&position).not().then(|| {
                                        self.positions.push(position);
                                    });
                                    ()
                                })
                            })
                            .ok_or(self.position.0.add(line_ix))
                    }) {
                        Ok(_) => None,
                        Err(line_ix) => {
                            self.direction = Direction::Left;
                            Some((line_ix, self.position.1))
                        }
                    }
                }),
            Direction::Left => {
                map.get(self.position.0)
                    .and_then(|line| line.get(0..self.position.1))
                    .and_then(|letters| letters.chars().collect::<Vec<_>>().into())
                    .and_then(|letters| {
                        match letters.iter().enumerate().rev().try_for_each(
                            |(letter_ix, letter)| {
                                letter
                                    .to_string()
                                    .ne("#")
                                    .then(|| {
                                        let position = (self.position.0, letter_ix);
                                        self.positions.contains(&position).not().then(|| {
                                            self.positions.push(position);
                                        });
                                        ()
                                    })
                                    .ok_or(letter_ix.add(1))
                            },
                        ) {
                            Ok(_) => None,
                            Err(letter_ix) => {
                                self.direction = Direction::Up;
                                Some((self.position.0, letter_ix))
                            }
                        }
                    })
            }
            Direction::Right => map
                .get(self.position.0)
                .and_then(|line| line.get(self.position.1.add(1)..line.len()))
                .and_then(|letters| letters.chars().collect::<Vec<_>>().into())
                .and_then(|letters| {
                    match letters
                        .iter()
                        .enumerate()
                        .try_for_each(|(letter_ix, letter)| {
                            letter
                                .to_string()
                                .ne("#")
                                .then(|| {
                                    let position =
                                        (self.position.0, self.position.1.add(letter_ix).add(1));
                                    self.positions.contains(&position).not().then(|| {
                                        self.positions.push(position);
                                    });
                                    ()
                                })
                                .ok_or(self.position.1.add(letter_ix))
                        }) {
                        Ok(_) => None,
                        Err(letter_ix) => {
                            self.direction = Direction::Down;
                            Some((self.position.0, letter_ix))
                        }
                    }
                }),
        };
        if let Some(position) = new_position {
            self.position = position;
            self.move_position(map)
        }
    }
}
