use helper::solved;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("in.txt");

fn roll_north(map: &mut Vec<Vec<u8>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'O' {
                let mut y = y;
                while y > 0 {
                    if map[y - 1][x] == b'.' {
                        map[y - 1][x] = b'O';
                        map[y][x] = b'.';
                        y -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn rotate(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut newmap = vec![vec![0; map.len()]; map[0].len()];
    for r in 0..map.len() {
        newmap
            .iter_mut()
            .enumerate()
            .take(map[0].len())
            .for_each(|(c, row)| {
                row[map.len() - 1 - r] = map[r][c];
            });
    }
    newmap
}

fn calc_load(map: &Vec<Vec<u8>>) -> usize {
    map.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .counts()
            .get(&b'O')
            .map_or(0, |&c| c * (map.len() - i))
    })
}

fn part_one(input: &str) -> usize {
    let mut map = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    roll_north(&mut map);
    calc_load(&map)
}

fn part_two(input: &str) -> usize {
    let mut map = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    fn cycle(map: &mut Vec<Vec<u8>>) {
        (0..4).for_each(|_| {
            roll_north(map);
            *map = rotate(map);
        });
    }

    let mut seen = HashMap::new();
    for i in 1..1000000000_u32 {
        cycle(&mut map);
        if let Some(seen_at) = seen.insert(map.clone(), i) {
            // the pattern repeats every i - seen_at iterations
            // so we can skip the first i iterations and then
            // calculate the remaining iterations modulo i - seen_at
            for _ in 0..(1000000000 - i) % (i - seen_at) {
                cycle(&mut map);
            }
            break;
        }
    }

    calc_load(&map)
}

fn main() {
    solved!("Day 14 part 1: {}", part_one(INPUT), 110565);
    solved!("Day 14 part 2: {}", part_two(INPUT), 89845);
}
