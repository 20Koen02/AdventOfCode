use colored::Colorize;
use itertools::Itertools;
use pathfinding::matrix::Matrix;

use crate::{get_enclosed, get_loop, parse};

pub fn visualize(input: &str) {
    let (start, full_maze) = parse(input);
    let shortest_loop = get_loop(start, &full_maze).collect_vec();
    let inclosed = get_enclosed(start, &full_maze).collect_vec();

    let mut clean_maze = Matrix::new(full_maze.rows, full_maze.columns, '.');
    for pos in &shortest_loop {
        clean_maze[*pos] = full_maze[*pos];
    }

    for (x, y) in clean_maze.keys() {
        let node = clean_maze[(x, y)];

        let unicode = match node {
            '|' => '│',
            '-' => '─',
            'L' => '└',
            'J' => '┘',
            '7' => '┐',
            'F' => '┌',
            '.' => ' ',
            _ => node,
        };

        if inclosed.contains(&(x, y)) {
            print!("{}", unicode.to_string().on_bright_yellow());
        } else if shortest_loop.contains(&(x, y)) {
            print!("{}", unicode.to_string().on_bright_red());
        } else {
            print!("{}", unicode.to_string().black());
        }

        if y == full_maze.columns - 1 {
            println!();
        }
    }
}
