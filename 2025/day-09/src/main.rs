use helper::solved;
use itertools::Itertools;
use rayon::prelude::*;

const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
const INPUT: &str = include_str!("in.txt");

type Point = (u64, u64);

#[derive(Copy, Clone)]
struct Rect {
    x1: u64,
    y1: u64,
    x2: u64,
    y2: u64,
}

impl Rect {
    #[inline]
    fn new(a: Point, b: Point) -> Self {
        let (x1, x2) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
        let (y1, y2) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
        Self { x1, y1, x2, y2 }
    }

    #[inline]
    fn area(&self) -> u64 {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1)
    }
}

#[derive(Copy, Clone)]
enum Segment {
    Vertical { x: u64, y1: u64, y2: u64 },
    Horizontal { y: u64, x1: u64, x2: u64 },
}

impl Segment {
    #[inline]
    fn new(a: Point, b: Point) -> Self {
        if a.0 == b.0 {
            let (y1, y2) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
            Segment::Vertical { x: a.0, y1, y2 }
        } else {
            let (x1, x2) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
            Segment::Horizontal { y: a.1, x1, x2 }
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[inline]
fn intersects(rect: Rect, seg: Segment) -> bool {
    let Rect { x1, y1, x2, y2 } = rect;

    match seg {
        Segment::Vertical {
            x,
            y1: sy1,
            y2: sy2,
        } => {
            if !(x1 < x && x2 > x) || sy2 == y1 || sy1 == y2 {
                return false;
            }
            (y1 >= sy1 && y1 <= sy2) || (y2 >= sy1 && y2 <= sy2)
        }
        Segment::Horizontal {
            y,
            x1: sx1,
            x2: sx2,
        } => {
            if !(y1 < y && y2 > y) || sx2 == x1 || sx1 == x2 {
                return false;
            }
            (x1 >= sx1 && x1 <= sx2) || (x2 >= sx1 && x2 <= sx2)
        }
    }
}

fn part_one(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| Rect::new(a, b).area())
        .max()
        .unwrap()
}

fn part_two(input: &str) -> u64 {
    let tiles = parse_input(input);

    let segments: Vec<Segment> = tiles
        .iter()
        .copied()
        .tuple_windows::<(_, _)>()
        .chain(std::iter::once((
            *tiles.last().unwrap(),
            *tiles.first().unwrap(),
        )))
        .map(|(a, b)| Segment::new(a, b))
        .collect();

    (0..tiles.len())
        .into_par_iter()
        .map(|i| {
            let a = tiles[i];
            let mut local_best = 0u64;
            for &b in &tiles[i + 1..] {
                let rect = Rect::new(a, b);
                let area = rect.area();
                if area <= local_best {
                    continue;
                }
                if !segments.iter().copied().any(|seg| intersects(rect, seg)) {
                    local_best = area;
                }
            }
            local_best
        })
        .max()
        .unwrap()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 50);
    solved!("Day 9 part one {}", part_one(INPUT), 4773451098u64);

    assert_eq!(part_two(TEST_INPUT), 24);
    solved!("Day 9 part two {}", part_two(INPUT), 1429075575);
}
