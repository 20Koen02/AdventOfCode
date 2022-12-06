use helper::solved;
use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

fn solve(n: usize) -> usize {
    INPUT
        .chars()
        .collect_vec()
        .windows(n)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
        .map(|(i, _)| i + n)
        .unwrap()
}

fn main() {
    solved!("Day 6 part 1: {}", solve(4), 1287);
    solved!("Day 6 part 2: {}", solve(14), 3716);
}
