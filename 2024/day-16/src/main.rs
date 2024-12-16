use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use helper::solved;

const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const INPUT: &str = include_str!("in.txt");

type IVec2 = (isize, isize);

fn find_node(grid: &[Vec<char>], char: char) -> IVec2 {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, c)| **c == char)
                .map(|(x, _)| (x as isize, y as isize))
        })
        .unwrap()
}

fn traverse(grid: &[Vec<char>], start: IVec2, end: IVec2) -> usize {
    let mut queue: BinaryHeap<Reverse<(usize, IVec2, IVec2)>> = BinaryHeap::new();
    queue.push(Reverse((0, start, (1, 0))));
    let mut seen: HashSet<(IVec2, IVec2)> = HashSet::new();

    while let Some(Reverse((cost, pos, dir))) = queue.pop() {
        if seen.contains(&(pos, dir)) {
            continue;
        }
        if pos == end {
            return cost;
        }
        seen.insert((pos, dir));

        let forwards = (pos.0 + dir.0, pos.1 + dir.1);
        if grid[forwards.1 as usize][forwards.0 as usize] != '#' {
            queue.push(Reverse((cost + 1, forwards, dir)));
        }
        queue.push(Reverse((cost + 1000, pos, (dir.1, -dir.0))));
        queue.push(Reverse((cost + 1000, pos, (-dir.1, dir.0))));
    }

    unreachable!()
}

fn walk(
    cur: Option<(IVec2, IVec2)>,
    routes: &mut HashSet<(IVec2, IVec2)>,
    tiles: &mut HashSet<IVec2>,
    links: &HashMap<(IVec2, IVec2), HashSet<Option<(IVec2, IVec2)>>>,
) {
    if let Some(cur) = cur {
        if !routes.contains(&cur) {
            routes.insert(cur);
            tiles.insert(cur.0);

            if let Some(link) = links.get(&cur) {
                for pos in link {
                    walk(*pos, routes, tiles, links);
                }
            }
        }
    }
}

fn traverse_all(grid: &[Vec<char>], start: IVec2, end: IVec2, target_cost: usize) -> usize {
    let mut best_costs: HashMap<(IVec2, IVec2), usize> = HashMap::new();
    let mut links: HashMap<(IVec2, IVec2), HashSet<Option<(IVec2, IVec2)>>> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(usize, IVec2, IVec2, Option<(IVec2, IVec2)>)>> = BinaryHeap::new();
    queue.push(Reverse((0, start, (1, 0), None)));

    while let Some(Reverse((cost, pos, dir, prev))) = queue.pop() {
        if cost > target_cost {
            break;
        }
        if best_costs.contains_key(&(pos, dir)) {
            if cost == best_costs[&(pos, dir)] {
                links.entry((pos, dir)).or_default().insert(prev);
            }
            continue;
        }

        best_costs.insert((pos, dir), cost);
        links.entry((pos, dir)).or_default().insert(prev);

        let prev = Some((pos, dir));
        let forwards = (pos.0 + dir.0, pos.1 + dir.1);
        if grid[forwards.1 as usize][forwards.0 as usize] != '#' {
            queue.push(Reverse((cost + 1, forwards, dir, prev)));
        }
        queue.push(Reverse((cost + 1000, pos, (dir.1, -dir.0), prev)));
        queue.push(Reverse((cost + 1000, pos, (-dir.1, dir.0), prev)));
    }

    let mut routes: HashSet<(IVec2, IVec2)> = HashSet::new();
    let mut tiles: HashSet<IVec2> = HashSet::new();

    for dir in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
        walk(Some((end, *dir)), &mut routes, &mut tiles, &links);
    }

    tiles.len()
}

fn solve(input: &str, part_two: bool) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_node(&grid, 'S');
    let end = find_node(&grid, 'E');

    let cost = traverse(&grid, start, end);

    if part_two {
        traverse_all(&grid, start, end, cost)
    } else {
        cost
    }
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 7036);
    solved!("Day 16 part one: {}", solve(INPUT, false), 85432);

    assert_eq!(solve(TEST_INPUT, true), 45);
    solved!("Day 16 part two: {}", solve(INPUT, true), 465);
}
