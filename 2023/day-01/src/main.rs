use helper::solved;

const TEST_INPUT_1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

const TEST_INPUT_2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

const INPUT: &str = include_str!("in.txt");
const SPELLED: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve(input: &str, part_two: bool) -> u32 {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut nums = line.chars().enumerate().filter_map(|(i, char)| match char {
                '0'..='9' => Some(char.to_digit(10).unwrap()),
                _ if part_two => SPELLED
                    .iter()
                    .enumerate()
                    .find_map(|(di, d)| line[i..].starts_with(d).then_some((di + 1) as u32)),
                _ => None,
            });

            let a = nums.next().unwrap();
            let b = nums.last().unwrap_or(a);
            a * 10 + b
        })
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT_1, false), 142);
    solved!("Day 1 part one: {}", solve(INPUT, false), 54708);

    assert_eq!(solve(TEST_INPUT_2, true), 281);
    solved!("Day 1 part two: {}", solve(INPUT, true), 54087);
}
