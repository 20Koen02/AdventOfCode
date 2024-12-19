use std::collections::{HashMap, HashSet};

use helper::solved;

const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
const INPUT: &str = include_str!("in.txt");

fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let (patterns_str, designs_str) = input.split_once("\n\n").unwrap();
    let patterns = patterns_str.split(", ").map(|s| s.to_string()).collect();
    let designs = designs_str.lines().map(|s| s.to_string()).collect();
    (patterns, designs)
}

fn designs_possible(
    design: &str,
    patterns: &HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    let count = patterns
        .iter()
        .filter(|&pattern| design.starts_with(pattern))
        .map(|pattern| designs_possible(&design[pattern.len()..], patterns, memo))
        .sum();

    memo.insert(design.to_string(), count);
    count
}

fn part_one(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    let mut memo = HashMap::new();

    designs
        .iter()
        .filter(|&design| designs_possible(design, &patterns, &mut memo) > 0)
        .count()
}

fn part_two(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    let mut memo = HashMap::new();

    designs
        .iter()
        .map(|design| designs_possible(design, &patterns, &mut memo))
        .sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 6);
    solved!("Day 19 part one: {}", part_one(INPUT), 313);

    assert_eq!(part_two(TEST_INPUT), 16);
    solved!("Day 19 part two: {}", part_two(INPUT), 666491493769758usize);
}
