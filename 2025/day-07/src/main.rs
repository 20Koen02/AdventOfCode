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
    beams[width / 2] = 1;

    let mut splits: u64 = 0;

    for &row in grid.iter().skip(1) {
        for c in 0..width {
            let k = beams[c];
            if row[c] == b'^' && k > 0 {
                splits += 1;

                beams[c] = 0;
                beams[c - 1] += k;
                beams[c + 1] += k;
            }
        }
    }

    (splits, beams.iter().sum::<u64>())
}

fn main() {
    assert_eq!(solve(TEST_INPUT).0, 21);
    solved!("Day 7 part one {}", solve(INPUT).0, 1553);

    assert_eq!(solve(TEST_INPUT).1, 40);
    solved!("Day 7 part two {}", solve(INPUT).1, 15811946526915u64);
}
