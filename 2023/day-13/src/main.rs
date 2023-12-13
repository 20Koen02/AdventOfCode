use std::vec;

use helper::solved;

const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

const INPUT: &str = include_str!("in.txt");

/// get the binary representation of all rows and columns in the pattern.
fn get_rows_cols(pattern: &str) -> (Vec<u32>, Vec<u32>) {
    let mut rows = vec![];
    let mut cols = vec![0; pattern.lines().next().unwrap().len()];
    for line in pattern.lines() {
        let mut row = 0;
        for (c, v) in line.bytes().enumerate() {
            cols[c] = (cols[c] << 1) | ((v == b'#') as u32);
            row = (row << 1) | ((v == b'#') as u32);
        }
        rows.push(row);
    }
    (rows, cols)
}

/// get the index of the reflection line, if any.
fn get_reflection(lines: &[u32], smudges: u32) -> Option<usize> {
    (1..lines.len()).find(|&i| {
        // check how many smudges there are if i is the reflection line
        // and compare it to the expected number of smudges
        (0..i)
            .rev()
            .zip(i..lines.len())
            .map(|(a, b)| (lines[a] ^ lines[b]).count_ones())
            .sum::<u32>()
            == smudges
    })
}

fn solve(input: &str, smudges: u32) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let (rows, cols) = get_rows_cols(pattern);
            get_reflection(&cols, smudges)
                .unwrap_or_else(|| get_reflection(&rows, smudges).unwrap() * 100)
        })
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, 0), 405);
    solved!("Day 13 part 1: {}", solve(INPUT, 0), 31265);
    assert_eq!(solve(TEST_INPUT, 1), 400);
    solved!("Day 13 part 2: {}", solve(INPUT, 1), 39359);
}
