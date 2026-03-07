use helper::solved;

const INPUT: &str = include_str!("in.txt");

fn part_one(input: &str) -> u64 {
    input
        .trim()
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .filter(|line| {
            let mut parts = line.split(": ");

            let area: u64 = parts
                .next()
                .unwrap()
                .split("x")
                .map(|s| s.parse::<u64>().unwrap())
                .product();

            let sum_counts: u64 = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .sum();

            area >= 9 * sum_counts
        })
        .count() as u64
}

fn main() {
    solved!("Day 12 part one {}", part_one(INPUT), 534);
}
