use std::collections::HashMap;

use helper::solved;
use itertools::Itertools;
use num::Integer;

const INPUT: &str = include_str!("in.txt");

#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    routes: HashMap<String, (String, String)>,
}

fn get_map(input: &str) -> Map {
    let lines = input.lines().collect_vec();
    Map {
        instructions: lines[0].chars().collect_vec(),
        routes: lines[2..]
            .iter()
            .map(|line| {
                let (key, value) = line.split_once(" = ").unwrap();
                let (mut left, mut right) = value.split_once(", ").unwrap();
                left = left.trim_start_matches('(');
                right = right.trim_end_matches(')');
                (key.to_string(), (left.to_string(), right.to_string()))
            })
            .collect(),
    }
}

fn part_one(input: &str) -> usize {
    let map = get_map(input);

    let mut curr = "AAA".to_string();
    let mut steps: usize = 0;
    while curr != "ZZZ" {
        let next = map.instructions[steps % map.instructions.len()];

        curr = if next == 'L' {
            map.routes[&curr].0.clone()
        } else {
            map.routes[&curr].1.clone()
        };
        steps += 1;
    }
    steps
}

fn part_two(input: &str) -> usize {
    let map = get_map(input);

    let start_nodes = map
        .routes
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect_vec();

    start_nodes
        .iter()
        .map(|pos| {
            let mut curr = pos.clone();
            let mut steps: usize = 0;
            while !curr.ends_with('Z') {
                let next = map.instructions[steps % map.instructions.len()];
                curr = if next == 'L' {
                    map.routes[&curr].0.clone()
                } else {
                    map.routes[&curr].1.clone()
                };
                steps += 1;
            }
            steps
        })
        .fold(1, |acc, steps| acc.lcm(&steps))
}

fn main() {
    solved!("Day 8 part 1: {}", part_one(INPUT), 16897);
    solved!("Day 8 part 2: {}", part_two(INPUT), 16563603485021_usize);
}
