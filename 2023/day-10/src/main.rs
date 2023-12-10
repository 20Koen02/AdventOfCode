mod visualize;

use std::iter;

use helper::solved;
use pathfinding::matrix::Matrix;

const INPUT: &str = include_str!("in.txt");

fn parse(input: &str) -> ((usize, usize), Matrix<char>) {
    let unpadded = Matrix::from_rows(input.lines().map(|s| s.chars())).unwrap();
    let mut maze = Matrix::new(unpadded.rows + 2, unpadded.columns + 2, '.');
    maze.set_slice((1, 1), &unpadded);

    for (x, y) in maze.keys() {
        if maze[(x, y)] == 'S' {
            // Get real shape of the start
            let mut bend = 0;
            bend |= (['|', 'F', '7'].contains(maze.get((x - 1, y)).unwrap()) as usize) << 3;
            bend |= (['|', 'L', 'J'].contains(maze.get((x + 1, y)).unwrap()) as usize) << 2;
            bend |= (['-', '7', 'J'].contains(maze.get((x, y + 1)).unwrap()) as usize) << 1;
            bend |= ['-', 'L', 'F'].contains(maze.get((x, y - 1)).unwrap()) as usize;
            maze[(x, y)] = "XXX-X7FXXJLX|XXX".chars().nth(bend).unwrap();

            return ((x, y), maze);
        }
    }
    panic!("No start found")
}

fn get_loop(
    start: (usize, usize),
    maze: &Matrix<char>,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut pos = start;
    let mut dir = match maze[start] {
        '|' | 'F' | '7' => (1, 0),
        'J' | 'L' => (-1, 0),
        _ => (0, 1),
    };
    iter::once(start).chain(iter::from_fn(move || {
        pos = maze.move_in_direction(pos, dir).unwrap();

        if pos == start {
            return None;
        }

        dir = match (dir, maze[pos]) {
            ((1, 0), 'L') | ((-1, 0), 'F') => (0, 1),
            ((1, 0), 'J') | ((-1, 0), '7') => (0, -1),
            ((0, 1), '7') | ((0, -1), 'F') => (1, 0),
            ((0, 1), 'J') | ((0, -1), 'L') => (-1, 0),
            _ => dir,
        };
        Some(pos)
    }))
}

fn get_enclosed(
    start: (usize, usize),
    maze: &Matrix<char>,
) -> impl Iterator<Item = (usize, usize)> {
    let mut clean_maze = Matrix::new(maze.rows, maze.columns, '.');
    for pos in get_loop(start, maze) {
        clean_maze[pos] = maze[pos];
    }

    let mut inside = false;
    clean_maze.keys().filter(move |(x, y)| {
        let node = clean_maze[(*x, *y)];

        // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
        // If the node is a . and we intersected an odd number of walls, we are inside the loop
        inside ^= ['|', 'J', 'L'].contains(&node);
        inside && node == '.'
    })
}

fn part_one(input: &str) -> usize {
    let (start, maze) = parse(input);
    get_loop(start, &maze).count() / 2
}

fn part_two(input: &str) -> usize {
    let (start, maze) = parse(input);
    get_enclosed(start, &maze).count()
}

fn main() {
    solved!("Day 10 part 1: {}", part_one(INPUT), 6968);
    solved!("Day 10 part 2: {}", part_two(INPUT), 413);

    visualize::visualize(INPUT);
}
