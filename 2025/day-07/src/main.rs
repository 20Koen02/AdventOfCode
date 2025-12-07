use std::mem;

use helper::solved;

const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
const INPUT: &str = include_str!("in.txt");

fn solve(input: &str) -> (u64, u64) {
    let grid: Vec<&[u8]> = input.trim().lines().map(|l| l.as_bytes()).collect();
    let width = grid[0].len();

    let mut beams = vec![0u64; width];
    let mut next = vec![0u64; width];
    let mut splits: u64 = 0;

    beams[width / 2] = 1;

    for &row in grid.iter().skip(1) {
        next.fill(0);

        for c in 0..width {
            let k = beams[c];
            if k == 0 {
                continue;
            }

            if row[c] == b'^' {
                splits += 1;
                next[c - 1] += k;
                next[c + 1] += k;
            } else {
                next[c] += k;
            }
        }

        mem::swap(&mut beams, &mut next);
    }

    (splits, beams.iter().sum::<u64>())
}

fn main() {
    assert_eq!(solve(TEST_INPUT).0, 21);
    solved!("Day 7 part one {}", solve(INPUT).0, 1553);

    assert_eq!(solve(TEST_INPUT).1, 40);
    solved!("Day 7 part two {}", solve(INPUT).1, 15811946526915u64);
}
