use cached::proc_macro::cached;
use std::{cmp::max, mem::swap};

const P1: usize = 6;
const P2: usize = 10;

// let die = iproduct!(1..4, 1..4, 1..4).map(|(d1, d2, d3)| d1 + d2 + d3);
const DIE: [usize; 27] = [
    3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
];

fn deterministic(mut p2: usize, mut p1: usize) -> usize {
    let mut s1: usize = 0;
    let mut s2: usize = 0;
    let mut die: usize = 1;
    let mut rolls: usize = 0;
    while s1 < 1000 {
        swap(&mut p1, &mut p2);
        swap(&mut s1, &mut s2);
        let v1 = (p1 + die * 3 + 3) % 10;
        p1 = if v1 == 0 { 10 } else { v1 };
        die = (die + 3) % 100;
        s1 += p1;
        rolls += 3;
    }
    rolls * s2
}

#[cached]
fn dirac(pos1: usize, pos2: usize, s1: usize, s2: usize) -> (usize, usize) {
    if s2 >= 21 {
        return (0, 1);
    }
    let mut score = (0, 0);
    for dice in DIE {
        let pos1 = pos1 + dice - (if pos1 + dice > 10 { 10 } else { 0 });
        let (s1, s2) = dirac(pos2, pos1, s2, s1 + pos1);
        score = (score.0 + s2, score.1 + s1);
    }
    score
}

fn main() {
    println!("Day 21 part one: {}", deterministic(P1, P2));
    let (s1, s2) = dirac(P1, P2, 0, 0);
    println!("Day 21 part two: {}", max(s1, s2));
}
