use helper::solved;
use rayon::prelude::*;
use std::collections::HashSet;

const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
const INPUT: &str = include_str!("in.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Node {
    Empty,
    Obstruction,
}

type Vector = (i32, i32, Direction);
type Board = Vec<Vec<Node>>;

fn parse(input: &str) -> (Vector, Board) {
    let mut guard = (0, 0, Direction::N);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, c)| match c {
                    b'#' => Node::Obstruction,
                    b'.' => Node::Empty,
                    b'^' => {
                        guard = (x as i32, y as i32, Direction::N);
                        Node::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (guard, map)
}

fn solve(mut guard: Vector, map: &Board) -> (u32, HashSet<Vector>, bool) {
    // All visited nodes for part one
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert((guard.0, guard.1));

    // Full path for part two
    let mut path = HashSet::<Vector>::new();
    // First direction for every visited node for part two
    let mut visited_dir = HashSet::<Vector>::new();

    let mut infinite_loop = false;

    loop {
        let (x, y, dir) = guard;

        let (new_x, new_y) = match dir {
            Direction::N => (x, y - 1),
            Direction::E => (x + 1, y),
            Direction::S => (x, y + 1),
            Direction::W => (x - 1, y),
        };

        match map
            .get(new_y as usize)
            .and_then(|row| row.get(new_x as usize))
        {
            Some(Node::Empty) => {
                guard = (new_x, new_y, dir);
                if visited.insert((new_x, new_y)) {
                    visited_dir.insert(guard);
                }
                if !(path.insert(guard)) {
                    infinite_loop = true;
                    break;
                }
            }
            Some(Node::Obstruction) => {
                guard = (x, y, dir.turn_right());
            }
            None => break,
        }
    }
    (visited.len() as u32, visited_dir, infinite_loop)
}

fn part_one(input: &str) -> u32 {
    let (guard, map) = parse(input);

    solve(guard, &map).0
}

fn part_two(input: &str) -> u32 {
    let (guard, map) = parse(input);

    solve(guard, &map)
        .1
        .into_par_iter()
        .filter(|&(x, y, d)| {
            let mut map_copy = map.to_vec();
            map_copy[y as usize][x as usize] = Node::Obstruction;

            // Go to previous node and turn right
            let r = match d {
                Direction::N => (x, y + 1, d.turn_right()),
                Direction::E => (x - 1, y, d.turn_right()),
                Direction::S => (x, y - 1, d.turn_right()),
                Direction::W => (x + 1, y, d.turn_right()),
            };

            solve(r, &map_copy).2
        })
        .count() as u32
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 41);
    solved!("Day 6 part one: {}", part_one(INPUT), 4819);

    assert_eq!(part_two(TEST_INPUT), 6);
    solved!("Day 6 part two: {}", part_two(INPUT), 1796);
}
