use helper::solved;

const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
const INPUT: &str = include_str!("in.txt");

fn solve(input: &str, k: usize) -> u64 {
    input
        .trim()
        .lines()
        .map(|bank| {
            let mut to_rem = bank.len() - k;
            let mut stack: Vec<u8> = Vec::with_capacity(bank.len());

            for d in bank.bytes().map(|b| b - b'0') {
                while !stack.is_empty() && to_rem > 0 && *stack.last().unwrap() < d {
                    stack.pop();
                    to_rem -= 1;
                }
                stack.push(d);
            }

            stack.truncate(k);

            stack.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
        })
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, 2), 357);
    solved!("Day 3 part one {}", solve(INPUT, 2), 17452);

    assert_eq!(solve(TEST_INPUT, 12), 3121910778619);
    solved!("Day 3 part two {}", solve(INPUT, 12), 173300819005913u64);
}
