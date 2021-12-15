use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = include_str!("in.txt");

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Node {
    risk: i32,
    pos: Pos,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_matrix() -> Vec<Vec<i32>> {
    INPUT
        .trim()
        .split("\n")
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

fn get_neighbours(p: &Pos) -> Vec<Pos> {
    vec![
        Pos { x: p.x - 1, y: p.y },
        Pos { x: p.x + 1, y: p.y },
        Pos { x: p.x, y: p.y + 1 },
        Pos { x: p.x, y: p.y - 1 },
    ]
}

fn dijkstra(map: HashMap<Pos, i32>, goal: Pos) -> i32 {
    let mut risks: HashMap<Pos, i32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(Node {
        risk: 0,
        pos: Pos { x: 0, y: 0 },
    });

    while let Some(node) = heap.pop() {
        if node.pos == goal {
            return node.risk;
        }
        if node.risk <= *risks.get(&node.pos).unwrap_or(&i32::MAX) {
            get_neighbours(&node.pos).iter().for_each(|&n| {
                if map.contains_key(&n) {
                    let next_square = Node {
                        risk: node.risk + map.get(&n).unwrap(),
                        pos: n,
                    };
                    if next_square.risk < *risks.get(&n).unwrap_or(&i32::MAX) {
                        heap.push(next_square);
                        risks.insert(n, next_square.risk);
                    }
                }
            });
        }
    }
    unreachable!()
}

fn get_map(matrix: &Vec<Vec<i32>>) -> HashMap<Pos, i32> {
    let mut map: HashMap<Pos, i32> = HashMap::new();
    matrix.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, risk)| {
            map.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                *risk,
            );
        })
    });
    return map;
}

fn get_map_multiplied(matrix: &Vec<Vec<i32>>) -> HashMap<Pos, i32> {
    let height = matrix.len() as i32;
    let width = matrix[0].len() as i32;
    let mut map: HashMap<Pos, i32> = HashMap::new();

    (0..5).for_each(|ym| {
        matrix.iter().enumerate().for_each(|(y, row)| {
            (0..5).for_each(|xm| {
                row.iter().enumerate().for_each(|(x, risk)| {
                    map.insert(
                        Pos {
                            x: x as i32 + xm * width,
                            y: y as i32 + ym * height,
                        },
                        ((risk + xm + ym) - 1) % 9 + 1,
                    );
                })
            })
        });
    });

    return map;
}

fn main() {
    let matrix = get_matrix();
    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;

    println!(
        "Day 15 part one: {}",
        dijkstra(get_map(&matrix), Pos { x: w - 1, y: h - 1 })
    );
    println!(
        "Day 15 part two: {}",
        dijkstra(
            get_map_multiplied(&matrix),
            Pos {
                x: w * 5 - 1,
                y: h * 5 - 1
            }
        )
    );
}
