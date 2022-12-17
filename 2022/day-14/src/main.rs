use std::collections::HashMap;

use helper::solved;

const INPUT: &str = include_str!("in.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Point {
    Air,
    Rock,
    Sand,
}

#[derive(Clone)]
struct Cave {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    points: HashMap<(i32, i32), Point>,
}

impl Default for Cave {
    fn default() -> Self {
        Self {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            points: Default::default(),
        }
    }
}

impl Cave {
    fn set(&mut self, x: i32, y: i32, point: Point) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);

        self.points.insert((x, y), point);
    }

    fn set_bounds(&mut self, y: i32) {
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }

    fn get(&self, x: i32, y: i32) -> Point {
        if y == (self.max_y + 2) {
            Point::Rock
        } else {
            self.points.get(&(x, y)).copied().unwrap_or(Point::Air)
        }
    }
}

fn get_cave() -> Cave {
    let mut cave = Cave::default();
    for line in INPUT.lines() {
        let points = line
            .split(" -> ")
            .map(|s| {
                let parts = s.split_once(',').unwrap();
                (parts.0.parse().unwrap(), parts.1.parse().unwrap())
            })
            .collect::<Vec<(i32, i32)>>();

        points.windows(2).for_each(|pair| {
            let a = pair[0];
            let b = pair[1];
            if a.0 == b.0 {
                for y in a.1.min(b.1)..=a.1.max(b.1) {
                    cave.set_bounds(y);
                    cave.set(a.0, y, Point::Rock);
                }
            } else if a.1 == b.1 {
                for x in a.0.min(b.0)..=a.0.max(b.0) {
                    cave.set_bounds(a.1);
                    cave.set(x, a.1, Point::Rock);
                }
            }
        })
    }
    cave
}

fn drop_sand(cave: &mut Cave) -> (i32, i32) {
    let mut x = 500;
    let mut y = 0;
    'down: loop {
        let ny = y + 1;

        for nx in [x, x - 1, x + 1] {
            if cave.get(nx, ny) == Point::Air {
                x = nx;
                y = ny;
                continue 'down;
            }
        }

        cave.set(x, y, Point::Sand);
        return (x, y);
    }
}

fn part1(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let mut ans = 0;
    while drop_sand(&mut cave).1 < cave.max_y {
        ans += 1;
    }
    ans
}

fn part2(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let mut ans = 1;
    while drop_sand(&mut cave) != (500, 0) {
        ans += 1;
    }
    ans
}

fn main() {
    let cave = get_cave();
    solved!("Day 14 part 1: {}", part1(&cave), 728);
    solved!("Day 14 part 2: {}", part2(&cave), 27623);
}
