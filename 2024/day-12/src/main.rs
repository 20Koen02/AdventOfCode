use std::collections::{HashSet, VecDeque};

use helper::solved;

const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

const INPUT: &str = include_str!("in.txt");

fn is_neighbour(grid: &Vec<Vec<u8>>, chr: u8, y: usize, x: usize) -> bool {
    *grid.get(y).and_then(|row| row.get(x)).unwrap_or(&0) == chr
}

fn perimeter(grid: &Vec<Vec<u8>>, shape: &HashSet<(usize, usize)>) -> usize {
    shape
        .iter()
        .map(|&(x, y)| {
            let mut count = 0;
            let chr = grid[y][x];

            count += (y == 0 || !is_neighbour(grid, chr, y - 1, x)) as usize;
            count += (y == grid.len() - 1 || !is_neighbour(grid, chr, y + 1, x)) as usize;
            count += (x == grid[0].len() - 1 || !is_neighbour(grid, chr, y, x + 1)) as usize;
            count += (x == 0 || !is_neighbour(grid, chr, y, x - 1)) as usize;

            count
        })
        .sum()
}

fn sides(shape: &HashSet<(usize, usize)>) -> usize {
    let mut seen = HashSet::new();
    for &(x, y) in shape {
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && shape.contains(&((nx) as usize, (ny) as usize)) {
                continue;
            }
            let (mut xx, mut yy) = (x, y);
            while (xx as isize + dy) >= 0
                && (yy as isize + dx) >= 0
                && shape.contains(&((xx as isize + dy) as usize, (yy as isize + dx) as usize))
                && !shape.contains(&((xx as isize + dx) as usize, (yy as isize + dy) as usize))
            {
                xx = (xx as isize + dy) as usize;
                yy = (yy as isize + dx) as usize;
            }
            seen.insert((xx, yy, dx, dy));
        }
    }
    seen.len()
}

fn get_shape(
    grid: &Vec<Vec<u8>>,
    y: usize,
    x: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::from([(x, y)]);
    let mut a = HashSet::from([(x, y)]);

    while let Some((xx, yy)) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = xx as isize + dx;
            let ny = yy as isize + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if is_neighbour(grid, grid[yy][xx], ny, nx) {
                if seen.insert((nx, ny)) {
                    a.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    a
}

fn solve(input: &str, part_two: bool) -> usize {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut seen = HashSet::new();
    let mut ans = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if seen.contains(&(x, y)) {
                continue;
            }
            if seen.insert((x, y)) {
                let shape = get_shape(&grid, y, x, &mut seen);
                if part_two {
                    ans += shape.len() * sides(&shape);
                } else {
                    ans += shape.len() * perimeter(&grid, &shape);
                }
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 1930);
    solved!("Day 12 part one: {}", solve(INPUT, false), 1449902);

    assert_eq!(solve(TEST_INPUT, true), 1206);
    solved!("Day 12 part two: {}", solve(INPUT, true), 908042);
}
