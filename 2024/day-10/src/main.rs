use std::collections::{HashSet, VecDeque};

use helper::solved;

const TEST_INPUT: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
const INPUT: &str = include_str!("in.txt");

fn solve(input: &str, part_two: bool) -> usize {
    let grid: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    let mut trailheads: Vec<(i32, i32, u8)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'0' {
                trailheads.push((x as i32, y as i32, b'0'));
            }
        }
    }

    trailheads
        .iter()
        .map(|t| {
            let mut queue = VecDeque::new();
            queue.push_back(*t);
            let mut unique = HashSet::new();
            let mut rating = 0;

            while let Some(pos) = queue.pop_front() {
                if pos.2 == b'9' {
                    unique.insert((pos.0, pos.1));
                    rating += 1;
                    continue;
                }
                for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let nx = pos.0 + d.0;
                    let ny = pos.1 + d.1;

                    match grid.get(ny as usize).and_then(|row| row.get(nx as usize)) {
                        Some(&d) if d == pos.2 + 1 => {
                            queue.push_back((nx, ny, d));
                        }
                        _ => {}
                    }
                }
            }

            if part_two {
                rating
            } else {
                unique.len()
            }
        })
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 36);
    solved!("Day 10 part one: {}", solve(INPUT, false), 786);

    assert_eq!(solve(TEST_INPUT, true), 81);
    solved!("Day 10 part two: {}", solve(INPUT, true), 1722);
}
