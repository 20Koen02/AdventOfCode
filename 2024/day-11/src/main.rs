use cached::proc_macro::cached;
use helper::solved;

const TEST_INPUT: &str = "125 17";
const INPUT: &str = include_str!("in.txt");

#[cached]
fn blink_count(stone: usize, iteration: usize, max_iterations: usize) -> usize {
    if iteration == max_iterations {
        return 1;
    }
    let stone_str = stone.to_string();
    if stone == 0 {
        blink_count(1, iteration + 1, max_iterations)
    } else if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);
        blink_count(left.parse().unwrap(), iteration + 1, max_iterations)
            + blink_count(right.parse().unwrap(), iteration + 1, max_iterations)
    } else {
        blink_count(stone * 2024, iteration + 1, max_iterations)
    }
}

fn solve(input: &str, max_iterations: usize) -> usize {
    input
        .split_whitespace()
        .map(|s| blink_count(s.parse().unwrap(), 0, max_iterations))
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, 25), 55312);
    solved!("Day 11 part one: {}", solve(INPUT, 25), 194557);

    assert_eq!(solve(TEST_INPUT, 75), 65601038650482);
    solved!(
        "Day 11 part two: {}",
        solve(INPUT, 75),
        231532558973909usize
    );
}
