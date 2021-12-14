use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("in.txt");

const START: &str = "PBVHVOCOCFFNBCNCCBHK";

fn get_rules() -> HashMap<(char, char), char> {
    return INPUT
        .lines()
        .map(|l| {
            let (pair, between) = l.split_once(" -> ").unwrap();
            let key = (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap());
            (key, between.chars().nth(0).unwrap())
        })
        .collect::<HashMap<(char, char), char>>();
}

fn solve(rules: &HashMap<(char, char), char>, steps: usize) -> usize {
    let start_pair_counts = START.chars().tuple_windows().counts();
    let pair_counts = (0..steps).fold(start_pair_counts, |counts, _| {
        let mut next = HashMap::new();
        counts.iter().for_each(|(&(left, right), count)| {
            let between = rules[&(left, right)];
            *next.entry((left, between)).or_default() += count;
            *next.entry((between, right)).or_default() += count;
        });
        return next;
    });

    let mut char_counts: HashMap<char, usize> = HashMap::new();
    pair_counts
        .iter()
        .for_each(|(&(left, _), count)| *char_counts.entry(left).or_default() += count);
    let last_char = START.chars().last().unwrap();
    *char_counts.entry(last_char).or_default() += 1;

    let max = char_counts.values().max().unwrap();
    let min = char_counts.values().min().unwrap();
    return max - min;
}

fn main() {
    let rules = get_rules();
    println!("Day 14 part one: {}", solve(&rules, 10));
    println!("Day 14 part two: {}", solve(&rules, 40));
}
