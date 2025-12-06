use helper::solved;

const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
const INPUT: &str = include_str!("in.txt");

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

#[inline]
fn can_access(grid: &[Vec<u8>], i: usize, j: usize) -> bool {
    let mut count = 0;
    let i = i as isize;
    let j = j as isize;

    for (di, dj) in NEIGHBORS {
        let ni = i + di;
        let nj = j + dj;
        if ni >= 0
            && ni < grid.len() as isize
            && nj >= 0
            && nj < grid[0].len() as isize
            && grid[ni as usize][nj as usize] == b'@'
        {
            count += 1;
            if count >= 4 {
                return false;
            }
        }
    }

    true
}

fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'@' && can_access(&grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn part_two(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut count = 0;
    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == b'@' && can_access(&grid, i, j) {
                    count += 1;
                    grid[i][j] = b'.';
                    changed = true;
                }
            }
        }
    }

    count
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 13);
    solved!("Day 4 part one {}", part_one(INPUT), 1451);

    assert_eq!(part_two(TEST_INPUT), 43);
    solved!("Day 4 part two {}", part_two(INPUT), 8701);
}
