use std::collections::HashMap;

use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
const INPUT: &str = include_str!("in.txt");

fn parse_input(input: &str, part_two: bool) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (
                calc_card_value(cards.chars().collect_vec().try_into().unwrap(), part_two),
                bid.parse().unwrap(),
            )
        })
        .collect()
}

fn calc_card_value(cards: [char; 5], part_two: bool) -> u32 {
    let mut value: u32 = 0;
    value |= get_suit_value(cards, part_two) << 20;
    value |= get_rank_value(cards, part_two);
    value
}

fn get_rank_value(cards: [char; 5], part_two: bool) -> u32 {
    cards.iter().enumerate().fold(0, |acc, (i, c)| {
        let value = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if part_two {
                    1
                } else {
                    11
                }
            }
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        };
        acc | value << (4 * (4 - i))
    })
}

fn get_suit_value(cards: [char; 5], part_two: bool) -> u32 {
    let mut counts: HashMap<char, u32> = HashMap::new();
    let mut joker_count = 0;

    for c in cards {
        if part_two && c == 'J' {
            joker_count += 1;
            continue;
        }
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let mut ident: Vec<u32> = counts.into_values().collect();
    ident.sort();

    if joker_count == 5 {
        return 6;
    }
    *ident.last_mut().unwrap() += joker_count;

    match ident.as_mut_slice() {
        [1, 1, 1, 1, 1] => 0,
        [1, 1, 1, 2] => 1,
        [1, 2, 2] => 2,
        [1, 1, 3] => 3,
        [2, 3] => 4,
        [1, 4] => 5,
        [5] => 6,
        _ => unreachable!(),
    }
}

fn solve(input: &str, part_two: bool) -> u32 {
    let mut hands = parse_input(input, part_two);
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, bid))| acc + bid * (idx as u32 + 1))
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 6440);
    solved!("Day 7 part one: {}", solve(INPUT, false), 249748283);
    assert_eq!(solve(TEST_INPUT, true), 5905);
    solved!("Day 7 part two: {}", solve(INPUT, true), 248029057);
}
