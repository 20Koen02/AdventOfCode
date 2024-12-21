use helper::solved;
use itertools::Itertools;
use std::collections::VecDeque;

const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
const INPUT: &str = include_str!("in.txt");

fn find_node(grid: &[Vec<char>], char: char) -> (isize, isize) {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| **c == char)
                .map(|(x, _)| (x as isize, y as isize))
        })
        .unwrap()
}

fn dist_to_point(grid: &[Vec<char>], point: (isize, isize)) -> Vec<Vec<Option<isize>>> {
    let mut queue = VecDeque::from([(0, point)]);
    let mut distances = vec![vec![None; grid[0].len() - 1]; grid.len() - 1];

    while let Some((distance, (x, y))) = queue.pop_front() {
        if distances[y as usize][x as usize].map_or(true, |d| d > distance) {
            distances[y as usize][x as usize] = Some(distance);

            for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if let Some(&nchar) = grid.get(ny as usize).and_then(|row| row.get(nx as usize)) {
                    if nchar != '#'
                        && distances[ny as usize][nx as usize].map_or(true, |d| d > distance + 1)
                    {
                        queue.push_back((distance + 1, (nx, ny)));
                    }
                }
            }
        }
    }

    distances
}

fn solve(input: &str, max_cheat_ps: isize, thresh: isize) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = find_node(&grid, 'S');
    let end = find_node(&grid, 'E');

    let dist_to_start = dist_to_point(&grid, start);
    let dist_to_end = dist_to_point(&grid, end);

    let base_score = dist_to_end[start.1 as usize][start.0 as usize].unwrap();

    let mut count = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let current_cell = grid[y][x];
            if current_cell == '#' || dist_to_end[y][x] == None {
                continue;
            }

            let base_steps = dist_to_start[y][x].unwrap();
            for dy in -max_cheat_ps..=max_cheat_ps {
                let ny = y as isize + dy;

                let remaining = max_cheat_ps - dy.abs();
                for dx in -remaining..=remaining {
                    let nx = x as isize + dx;

                    if let Some(&nchar) = grid.get(ny as usize).and_then(|row| row.get(nx as usize))
                    {
                        if nchar == '#' || dist_to_start[ny as usize][nx as usize] == None {
                            continue;
                        }

                        let new_distance = dist_to_end[ny as usize][nx as usize].unwrap();
                        let cheated_distance = dy.abs() + dx.abs();

                        let score = base_steps + cheated_distance + new_distance;
                        let savings = base_score - score;

                        if savings >= thresh {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn main() {
    assert_eq!(solve(TEST_INPUT, 2, 12), 8);
    solved!("Day 20 part one: {}", solve(INPUT, 2, 100), 1197);

    assert_eq!(solve(TEST_INPUT, 20, 68), 55);
    solved!("Day 20 part two: {}", solve(INPUT, 20, 100), 944910);
}
