use std::ops::Sub;

use helper::solved;
use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Sub for Point {
    type Output = usize;

    fn sub(self, other: Self) -> Self::Output {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

/// Get the coordinates of all the galaxies
fn get_coords(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .fold(vec![], |mut acc, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    acc.push(Point {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            });
            acc
        })
}

/// Get the indexes of the empty rows/columns
fn get_empty_idx(idx: Vec<bool>) -> Vec<usize> {
    idx.iter()
        .enumerate()
        .filter_map(|(i, v)| if *v { Some(i) } else { None })
        .collect_vec()
}

/// Expand the empty rows/columns
fn expand_coords(mut coords: Vec<Point>, part_two: bool) -> Vec<Point> {
    let max_y = coords.iter().max_by_key(|p| p.y).unwrap().y;
    let max_x = coords.iter().max_by_key(|p| p.x).unwrap().x;
    let mut rows = vec![true; max_y as usize + 1];
    let mut cols = vec![true; max_x as usize + 1];

    coords.iter().for_each(|p| {
        rows[p.y as usize] = false;
        cols[p.x as usize] = false;
    });

    let expand_row_idx = get_empty_idx(rows);
    let expand_col_idx = get_empty_idx(cols);

    coords.iter_mut().for_each(|p| {
        let move_x = expand_col_idx.iter().filter(|i| **i < p.x as usize).count() as isize;
        let move_y = expand_row_idx.iter().filter(|i| **i < p.y as usize).count() as isize;

        p.y += if part_two {
            move_y * 1_000_000 - move_y
        } else {
            move_y
        };
        p.x += if part_two {
            move_x * 1_000_000 - move_x
        } else {
            move_x
        };
    });

    coords
}

fn solve(input: &str, part_two: bool) -> usize {
    let coords = get_coords(input);
    let expanded = expand_coords(coords, part_two);

    expanded
        .iter()
        .tuple_combinations::<(&Point, &Point)>()
        .fold(0, |acc, (a, b)| {
            let diff = *a - *b;
            acc + diff
        })
}

fn main() {
    solved!("Day 11 part 1: {}", solve(INPUT, false), 10033566);
    solved!("Day 11 part 2: {}", solve(INPUT, true), 560822911938_usize);
}
