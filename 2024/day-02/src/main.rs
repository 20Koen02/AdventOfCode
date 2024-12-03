use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
const INPUT: &str = include_str!("in.txt");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn is_safe(report: &Vec<u32>) -> bool {
    let is_increasing = report[1] > report[0] && report[2] > report[1];

    let invalid_adj_lvls = |a: u32, b: u32| -> bool {
        if is_increasing {
            a < b
        } else {
            a > b
        }
    };

    for i in 1..report.len() {
        let diff = report[i].abs_diff(report[i - 1]);
        if invalid_adj_lvls(report[i], report[i - 1]) || ![1, 2, 3].contains(&diff) {
            return false;
        }
    }
    true
}

fn is_safe_dampened(report: &Vec<u32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report = report.clone();
        report.remove(i);
        if is_safe(&report) {
            return true;
        }
    }
    false
}

fn solve(input: &str, part_two: bool) -> u32 {
    let reports = parse_input(input);

    reports
        .iter()
        .filter(|&report| {
            if part_two {
                is_safe_dampened(report)
            } else {
                is_safe(report)
            }
        })
        .count() as u32
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 2);
    solved!("Day 2 part one: {}", solve(INPUT, false), 356);

    assert_eq!(solve(TEST_INPUT, true), 4);
    solved!("Day 2 part two: {}", solve(INPUT, true), 413);
}
