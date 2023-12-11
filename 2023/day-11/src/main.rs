use helper::solved;
use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

type Point = (usize, usize);

fn get_coords(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .fold(vec![], |mut acc, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' { acc.push((x, y)); }
            });
            acc
        })
}

fn get_empty_idx(idx: Vec<bool>) -> Vec<usize> {
    idx.iter()
        .enumerate()
        .filter_map(|(i, v)| if *v { Some(i) } else { None })
        .collect_vec()
}

fn expand_coords(coords: Vec<Point>, multiplier: usize) -> Vec<Point> {
    let max_y = coords.iter().max_by_key(|p| p.1).unwrap().1;
    let max_x = coords.iter().max_by_key(|p| p.0).unwrap().0;
    let mut rows = vec![true; max_y + 1];
    let mut cols = vec![true; max_x + 1];

    coords.iter().for_each(|p| {
        rows[p.1] = false;
        cols[p.0] = false;
    });

    let expand_row_idx = get_empty_idx(rows);
    let expand_col_idx = get_empty_idx(cols);

    coords
        .iter()
        .map(|p| {
            let move_y = expand_row_idx.iter().filter(|i| **i < p.1).count();
            let move_x = expand_col_idx.iter().filter(|i| **i < p.0).count();
            (p.0 + move_x * multiplier, p.1 + move_y * multiplier)
        })
        .collect_vec()
}

fn solve(input: &str, multiplier: usize) -> usize {
    let coords = get_coords(input);
    let expanded = expand_coords(coords, multiplier);

    expanded
        .iter()
        .tuple_combinations::<(&Point, &Point)>()
        .fold(0, |acc, (a, b)| {
            let diff = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
            acc + diff
        })
}

fn main() {
    solved!("Day 11 part 1: {}", solve(INPUT, 1), 10033566);
    solved!("Day 11 part 2: {}", solve(INPUT, 999_999), 560822911938_usize);
}
