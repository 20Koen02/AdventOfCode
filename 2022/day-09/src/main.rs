use std::collections::HashSet;

use helper::solved;

const INPUT: &str = include_str!("in.txt");

type Move = ((i32, i32), u32);

fn get_moves() -> Vec<Move> {
    INPUT
        .lines()
        .map(|l| {
            let (direction, times) = l.split_once(' ').unwrap();
            (
                // Kernel for the 4 directions
                match direction {
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    "U" => (0, 1),
                    "D" => (0, -1),
                    _ => unreachable!(),
                },
                times.parse().unwrap(),
            )
        })
        .collect()
}

fn solve(moves: &Vec<Move>, length: usize) -> usize {
    let mut nodes = vec![[0, 0]; length];
    let mut visited = HashSet::from([(0, 0)]);

    for &(kernel, times) in moves {
        for _ in 0..times {
            // Move the head by applying the kernel
            nodes[0][0] += kernel.0;
            nodes[0][1] += kernel.1;

            // Move all the other nodes to the head
            for i in 1..nodes.len() {
                let head = nodes[i - 1];
                let dx: i32 = head[0] - nodes[i][0];
                let dy: i32 = head[1] - nodes[i][1];

                // If the distance between the node and the head is 2, the node is moved by 1 unit in the direction of the head.
                if dx.abs() == 2 || dy.abs() == 2 {
                    nodes[i][0] += dx.signum();
                    nodes[i][1] += dy.signum();

                    // If i is the last node (the tail), the node is added to the visited set
                    if i == nodes.len() - 1 {
                        visited.insert((nodes[i][0], nodes[i][1]));
                    }
                }
            }
        }
    }

    visited.len()
}

fn main() {
    let moves: Vec<Move> = get_moves();
    solved!("Day 9 part 1: {}", solve(&moves, 2), 6486);
    solved!("Day 9 part 2: {}", solve(&moves, 10), 2678);
}
