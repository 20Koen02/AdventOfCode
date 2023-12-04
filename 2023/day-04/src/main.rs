use std::collections::HashSet;
use itertools::Itertools;
use helper::solved;

const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
const INPUT: &str = include_str!("in.txt");

type Card = (HashSet<u32>, HashSet<u32>);

fn get_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            line.split(": ")
                .nth(1)
                .unwrap()
                .split(" | ")
                .map(|part| {
                    part.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    let cards = get_cards(input);

    cards
        .iter()
        .map(|(a, b)| {
            let count = a.intersection(b).count();
            if count == 0 { 0 } else { 2u32.pow(count as u32 - 1) }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let cards = get_cards(input);
    let mut copies: Vec<u32> = vec![1; cards.len()];

    for (i, (a, b)) in cards.iter().enumerate() {
        let winning = a.intersection(b);
        let current_copies = copies[i];
        for c in copies.iter_mut().skip(i + 1).take(winning.count()) {
            *c += current_copies
        }
    }

    copies.iter().sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 13);
    solved!("Day 4 part 1: {}", part_one(INPUT), 21158);
    assert_eq!(part_two(TEST_INPUT), 30);
    solved!("Day 4 part 2: {}", part_two(INPUT), 6050769);
}
