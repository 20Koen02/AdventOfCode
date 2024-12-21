use cached::proc_macro::cached;
use std::{cmp, iter::once};

use helper::solved;

const INPUT: &str = include_str!("in.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IVec2(isize, isize);

impl IVec2 {
    fn numpad(key: char) -> Self {
        match key {
            '7' => Self(0, 0), '8' => Self(1, 0), '9' => Self(2, 0),
            '4' => Self(0, 1), '5' => Self(1, 1), '6' => Self(2, 1),
            '1' => Self(0, 2), '2' => Self(1, 2), '3' => Self(2, 2),
                               '0' => Self(1, 3), 'A' => Self(2, 3),
            _ => unreachable!(),
        }
    }

    fn dirpad(key: char) -> Self {
        match key {
                               '^' => Self(1, 0), 'A' => Self(2, 0),
            '<' => Self(0, 1), 'v' => Self(1, 1), '>' => Self(2, 1),
            _ => unreachable!(),
        }
    }

    fn rel_dist(self, other: Self) -> IVec2 {
        IVec2(self.0 - other.0, self.1 - other.1)
    }
}

#[cached]
fn shortest_dirpad(dist: IVec2, remaining: usize, hor_first: bool) -> usize {
    if remaining == 0 {
        return dist.0.abs() as usize + dist.1.abs() as usize + 1;
    }

    let ver_vec = vec![if dist.1 > 0 { '^' } else { 'v' }; dist.1.abs() as usize];
    let hor_vec = vec![if dist.0 > 0 { '<' } else { '>' }; dist.0.abs() as usize];

    let button_iter = (if hor_first { &hor_vec } else { &ver_vec })
        .into_iter()
        .chain((if hor_first { &ver_vec } else { &hor_vec }).into_iter())
        .chain(once(&'A'));

    let mut start = IVec2::dirpad('A');

    button_iter
        .map(|&c| {
            let end = IVec2::dirpad(c);
            let dist = IVec2::rel_dist(start, end);

            let start_copy = start;
            start = end;

            if dist == IVec2(0, 0) {
                return 1;
            }

            match (start_copy, end) {
                (IVec2(_, 0), IVec2(0, 1)) => shortest_dirpad(dist, remaining - 1, false),
                (IVec2(0, 1), IVec2(_, 0)) => shortest_dirpad(dist, remaining - 1, true),
                _ => cmp::min(
                    shortest_dirpad(dist, remaining - 1, false),
                    shortest_dirpad(dist, remaining - 1, true),
                ),
            }
        })
        .sum()
}

fn shortest_numpad(code: &str, robots: usize) -> usize {
    let mut start = IVec2::numpad('A');

    code[0..3].parse::<usize>().unwrap()
        * code
            .chars()
            .map(|c| {
                let end = IVec2::numpad(c);
                let dist = IVec2::rel_dist(start, end);

                let start_copy = start;
                start = end;

                match (start_copy, end) {
                    (IVec2(_, 3), IVec2(0, _)) => shortest_dirpad(dist, robots, false),
                    (IVec2(0, _), IVec2(_, 3)) => shortest_dirpad(dist, robots, true),
                    _ => cmp::min(
                        shortest_dirpad(dist, robots, true),
                        shortest_dirpad(dist, robots, false),
                    ),
                }
            })
            .sum::<usize>()
}

fn solve(input: &str, robots: usize) -> usize {
    input
        .lines()
        .map(|code| shortest_numpad(code, robots))
        .sum()
}

fn main() {
    solved!("Day 21 part one: {}", solve(INPUT, 2), 171596);
    solved!("Day 21 part two: {}", solve(INPUT, 25), 209268004868246usize);
}
