use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
const INPUT: &str = include_str!("in.txt");

fn solve_history(numbers: Vec<i32>, part_two: bool) -> i32 {
    // Base case
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }

    let differences = (1..numbers.len())
        .map(|i| numbers[i] - numbers[i - 1])
        .collect_vec();

    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();

    if part_two {
        first - solve_history(differences, true)
    } else {
        solve_history(differences, false) + last
    }
}

fn solve(input: &str, part_two: bool) -> i32 {
    input
        .lines()
        .map(|line| {
            solve_history(
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec(),
                part_two,
            )
        })
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 114);
    solved!("Day 9 part 1: {}", solve(INPUT, false), 1834108701);
    assert_eq!(solve(TEST_INPUT, true), 2);
    solved!("Day 9 part 2: {}", solve(INPUT, true), 993);
}
