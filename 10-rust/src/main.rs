const INPUT: &str = include_str!("in.txt");
const OPEN_TAGS: &str = "([{<";
const CLOSE_TAGS: &str = ")]}>";

fn main() {
    let blocks: Vec<String> = INPUT.trim().split("\n").map(String::from).collect();
    let mut day_one = 0;
    let mut day_two_scores: Vec<usize> = blocks
        .iter()
        .filter_map(|block| {
            let mut stack = Vec::new();
            for c in block.chars() {
                if let Some(i) = CLOSE_TAGS.chars().position(|p| c == p) {
                    if stack.pop() != OPEN_TAGS.chars().nth(i) {
                        day_one += [3, 57, 1197, 25137][i];
                        return None;
                    }
                } else {
                    stack.push(c);
                }
            }
            return Some(stack.iter().rev().fold(0, |acc, &c| {
                acc * 5 + OPEN_TAGS.chars().position(|p| c == p).unwrap() + 1
            }));
        })
        .collect::<Vec<usize>>();

    day_two_scores.sort();
    let day_two = day_two_scores[day_two_scores.len() / 2];
    println!("Day 10 part one: {}", day_one);
    println!("Day 10 part two: {}", day_two);
}
