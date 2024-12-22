use helper::solved;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("in.txt");

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mut s| {
            for _ in 0..2000 {
                s = ((s * 64) ^ s) % 16777216;
                s = (s / 32) ^ s;
                s = ((s * 2048) ^ s) % 16777216;
            }
            s
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut bananas = HashMap::<[isize; 4], usize>::new();

    for mut s in input.lines().map(|line| line.parse::<usize>().unwrap()) {
        let mut sequence: [isize; 4] = [0; 4];
        let mut seen = HashSet::<[isize; 4]>::new();

        for i in 0..2000 {
            let prev = (s % 10) as isize;

            s = ((s * 64) ^ s) % 16777216;
            s = (s / 32) ^ s;
            s = ((s * 2048) ^ s) % 16777216;

            sequence.rotate_right(1);
            sequence[0] = prev - (s % 10) as isize;

            if i >= 3 {
                if seen.insert(sequence) {
                    bananas
                        .entry(sequence)
                        .and_modify(|e| *e += s % 10)
                        .or_insert(s % 10);
                }
            }
        }
    }

    *bananas.iter().map(|(_, v)| v).max().unwrap()
}

fn main() {
    solved!("Day 22 part one: {}", part_one(INPUT), 13764677935usize);
    solved!("Day 22 part two: {}", part_two(INPUT), 1619);
}
