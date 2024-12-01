use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
const INPUT: &str = include_str!("in.txt");

fn get_left_right(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn part_one(input: &str) -> u32 {
    let (mut left, mut right) = get_left_right(input);

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part_two(input: &str) -> u32 {
    let (left, right) = get_left_right(input);

    left.iter()
        .map(|a| a * right.iter().filter(|&b| a == b).count() as u32)
        .sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 11);
    solved!("Day 1 part one: {}", part_one(INPUT), 2196996);

    assert_eq!(part_two(TEST_INPUT), 31);
    solved!("Day 1 part two: {}", part_two(INPUT), 23655822);
}
