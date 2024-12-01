/// [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)
/// What is the total distance between your lists?
pub fn solve(path: String) {
    let (mut left, mut right) = std::fs::read_to_string(path)
        .expect("Input file should exist")
        .lines()
        .map(
            |line| match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                [first, second, ..] => (
                    first
                        .parse::<u32>()
                        .expect("First line token should be a number"),
                    second
                        .parse::<u32>()
                        .expect("Right line token should be a number"),
                ),
                _ => panic!("Invalid input: a given line must have at least 2 elements"),
            },
        )
        .unzip::<_, _, Vec<u32>, Vec<u32>>();
    left.sort();
    right.sort();
    let result = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .fold(0, |acc, x| std::ops::Add::add(acc, x));
    println!("Part 1: {}", result);
}
