use helper::solved;

const INPUT: &str = include_str!("in.txt");
const SPELLED: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve(part_two: bool) -> u32 {
    INPUT
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
    solved!("Day 1 part one: {}", solve(false), 54708);
    solved!("Day 1 part two: {}", solve(true), 54087);
}
