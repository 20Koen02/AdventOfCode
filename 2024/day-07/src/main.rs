use helper::solved;
use rayon::prelude::*;

const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
const INPUT: &str = include_str!("in.txt");

const ADD: fn(usize, usize) -> usize = |a, b| a + b;
const MULTIPLY: fn(usize, usize) -> usize = |a, b| a * b;
const CONCAT: fn(usize, usize) -> usize = |a, b| (a.to_string() + &b.to_string()).parse().unwrap();
const OPS_ONE: &[fn(usize, usize) -> usize] = &[ADD, MULTIPLY];
const OPS_TWO: &[fn(usize, usize) -> usize] = &[ADD, MULTIPLY, CONCAT];

type Equation = (usize, Vec<usize>);

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (test, numbers) = line.split_once(": ").unwrap();
            (
                test.parse().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn eval(numbers: &[usize], test: usize, part_two: bool) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == test;
    }

    let ops = if part_two { OPS_TWO } else { OPS_ONE };

    for op in ops {
        let value = op(numbers[0], numbers[1]);

        let mut new_numbers = numbers[2..].to_vec();
        new_numbers.insert(0, value);

        if eval(&new_numbers, test, part_two) {
            return true;
        }
    }
    false
}

fn solve(input: &str, part_two: bool) -> usize {
    let equations = parse(input);

    equations
        .par_iter()
        .map(|(test, numbers)| {
            if eval(numbers, *test, part_two) {
                *test
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 3749);
    solved!("Day 7 part one: {}", solve(INPUT, false), 538191549061usize);

    assert_eq!(solve(TEST_INPUT, true), 11387);
    solved!(
        "Day 7 part two: {}",
        solve(INPUT, true),
        34612812972206usize
    );
}
