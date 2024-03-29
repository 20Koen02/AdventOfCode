use std::collections::HashMap;

use helper::solved;

const INPUT: &str = include_str!("in.txt");

fn solve() -> HashMap<String, u32> {
    let mut paths_traveled: Vec<String> = vec!["/".to_string()];
    let mut path_sizes: HashMap<String, u32> = HashMap::new();

    for cmd in INPUT.lines().skip(1) {
        let parts: Vec<&str> = cmd.split(' ').collect();
        match parts.as_slice() {
            ["dir", ..] | ["$", "ls"] => {}
            ["$", "cd", ".."] => {
                paths_traveled.pop();
            }
            ["$", "cd", dir] => {
                paths_traveled.push(paths_traveled.join("/") + "/" + dir);
            }
            [size_str, ..] => {
                let size = size_str.parse::<u32>().unwrap();
                for path in &paths_traveled {
                    *path_sizes.entry(path.to_string()).or_insert(0) += size
                }
            }
            [..] => unreachable!(),
        };
    }

    path_sizes
}

fn part1(path_sizes: &HashMap<String, u32>) -> u32 {
    path_sizes
        .values()
        .into_iter()
        .filter(|&n| *n <= 100_000)
        .sum()
}

fn part2(path_sizes: &HashMap<String, u32>) -> u32 {
    let remaining = 30_000_000 - (70_000_000 - path_sizes.get("/").unwrap());
    *path_sizes
        .values()
        .into_iter()
        .filter(|&n| *n >= remaining)
        .min()
        .unwrap()
}

fn main() {
    let path_sizes = solve();
    solved!("Day 7 part 1: {}", part1(&path_sizes), 1444896);
    solved!("Day 7 part 2: {}", part2(&path_sizes), 404395);
}
