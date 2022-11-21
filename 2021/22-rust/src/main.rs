use std::{
    cmp::{max, min},
    collections::HashMap,
    iter::zip,
};

const INPUT: &str = include_str!("in.txt");

type CuboidBounds = [(i64, i64); 3];
type Cuboid = (bool, CuboidBounds);

fn get_cuboids() -> Vec<Cuboid> {
    return INPUT
        .trim()
        .lines()
        .map(|line| -> Cuboid {
            let (on_str, coords) = line.split_once(' ').unwrap();
            let coords: CuboidBounds = coords
                .split(',')
                .map(|c| {
                    let (a, b) = c[2..].split_once("..").unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (on_str == "on", coords)
        })
        .collect();
}

fn volume(bounds: CuboidBounds) -> u64 {
    bounds.iter().fold(1, |acc, (lower, upper)| {
        acc * (i64::unsigned_abs(upper - lower) + 1)
    })
}

fn intersect(bounds1: CuboidBounds, bounds2: CuboidBounds) -> Option<CuboidBounds> {
    let mut ans: Vec<(i64, i64)> = vec![];
    // calculate intersection for every axis
    for (b1, b2) in zip(bounds1, bounds2) {
        // if one of the axis does not intersect, the cuboid also doesn't intersect
        if b1.1 < b2.0 || b2.1 < b1.0 {
            return None;
        }
        let intersect = (max(b1.0, b2.0), min(b1.1, b2.1));
        ans.push(intersect)
    }
    Some(ans.try_into().unwrap())
}

fn solve(cuboids: Vec<Cuboid>) -> i64 {
    let mut counts: HashMap<CuboidBounds, i64> = HashMap::new();
    for (on, bounds) in cuboids {
        let mut new_counts: HashMap<CuboidBounds, i64> = HashMap::new();
        for (intersection, val) in &counts {
            let new_intersect = intersect(bounds, *intersection);
            match new_intersect {
                Some(i) => *new_counts.entry(i).or_insert(0) -= val,
                None => continue,
            };
        }

        if on {
            *new_counts.entry(bounds).or_insert(0) += 1;
        }

        for (c, val) in new_counts {
            *counts.entry(c).or_insert(0) += val;
        }
    }

    counts
        .iter()
        .fold(0, |acc, (c, val)| acc + (volume(*c) as i64 * val))
}

fn main() {
    let cuboids = get_cuboids();
    println!("Day 22 part one: {}", solve(cuboids[0..20].to_vec()));
    println!("Day 22 part two: {}", solve(cuboids));
}
