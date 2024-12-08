use std::collections::HashSet;

use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

const INPUT: &str = include_str!("in.txt");

fn solve(input: &str, part_two: bool) -> usize {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let freq_map: Vec<Vec<(i32, i32)>> = (b'0'..=b'9')
        .chain(b'A'..=b'Z')
        .chain(b'a'..=b'z')
        .map(|c| {
            grid.iter()
                .enumerate()
                .filter_map(|(y, row)| {
                    row.iter().enumerate().find_map(|(x, cell)| {
                        if *cell == c {
                            Some((x as i32, y as i32))
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .collect();

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    let len_x = grid[0].len() as i32;
    let len_y = grid.len() as i32;

    for freq in freq_map.iter() {
        for pair in freq.iter().combinations(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            let dx = x2 - x1;
            let dy = y2 - y1;

            let mut an = vec![(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)];

            if part_two {
                let mut i = 2;
                loop {
                    let mut oob = true;

                    let new_x1 = x1 - dx * i;
                    let new_y1 = y1 - dy * i;
                    if new_x1 >= 0 && new_x1 < len_x && new_y1 >= 0 && new_y1 < len_y {
                        an.push((new_x1, new_y1));
                        oob = false;
                    }

                    let new_x2 = x2 + dx * i;
                    let new_y2 = y2 + dy * i;
                    if new_x2 >= 0 && new_x2 < len_x && new_y2 >= 0 && new_y2 < len_y {
                        an.push((new_x2, new_y2));
                        oob = false;
                    }

                    if oob {
                        break;
                    }

                    i += 1;
                }
                an.extend(pair);
            }

            for (x, y) in an {
                if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
                    antinodes.insert((x, y));
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 14);
    solved!("Day 8 part one: {}", solve(INPUT, false), 329);

    assert_eq!(solve(TEST_INPUT, true), 34);
    solved!("Day 8 part two: {}", solve(INPUT, true), 1190);
}
