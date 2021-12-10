const INPUT: &str = include_str!("in.txt");
const OPEN_TAGS: &str = "([{<";
const CLOSE_TAGS: &str = ")]}>";

fn main() {
    let blocks: Vec<String> = INPUT.trim().split("\n").map(String::from).collect();
    let mut part_one = 0;
    let mut part_two_scores: Vec<usize> = blocks
        .iter()
        .filter_map(|block| {
            let mut stack = Vec::new();
            for c in block.chars() {
                if let Some(i) = CLOSE_TAGS.chars().position(|p| c == p) {
                    if stack.pop() != OPEN_TAGS.chars().nth(i) {
                        part_one += [3, 57, 1197, 25137][i];
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

    part_two_scores.sort();
    let part_two = part_two_scores[part_two_scores.len() / 2];
    println!("part 10 part one: {}", part_one);
    println!("part 10 part two: {}", part_two);
}
