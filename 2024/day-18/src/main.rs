use std::collections::{HashSet, VecDeque};

use helper::solved;

const INPUT: &str = include_str!("in.txt");

type IVec2 = (isize, isize);

fn parse(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<_>>()
}

fn bfs(corrupted: &[IVec2]) -> Option<usize> {
    let end = (70, 70);
    let mut queue: VecDeque<(IVec2, usize)> = VecDeque::from([((0, 0), 0)]);
    let mut visited: HashSet<IVec2> = HashSet::from([(0, 0)]);

    while let Some((pos, step)) = queue.pop_front() {
        for npos in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            if npos.0 >= 0 && npos.0 <= end.0 && npos.1 >= 0 && npos.1 <= end.1 {
                if npos == end {
                    return Some(step + 1);
                }

                if !corrupted.contains(&&npos) && !visited.contains(&npos) {
                    queue.push_back((npos, step + 1));
                    visited.insert(npos);
                }
            }
        }
    }
    None
}

fn part_one(input: &str) -> usize {
    let corrupted = parse(input);
    bfs(&corrupted[..1024]).unwrap()
}

fn part_two(input: &str) -> String {
    let corrupted = parse(input);

    let mut l = 1024;
    let mut r = corrupted.len();

    while l <= r {
        let m = (l + r) / 2;
        if bfs(&corrupted[..m]).is_some() {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    let ans = corrupted[l - 1];
    format!("{},{}", ans.0, ans.1)
}

fn main() {
    solved!("Day 18 part one: {}", part_one(INPUT), 282);
    solved!("Day 18 part two: {}", part_two(INPUT), "64,29");
}
