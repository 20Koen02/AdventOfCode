use helper::solved;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("in.txt");

// roll the map in the given direction
// y_dir: -1 = north, 1 = south
// x_dir: -1 = west, 1 = east
fn roll(map: &mut Vec<Vec<u8>>, y_dir: isize, x_dir: isize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // reverse iteration if y_dir or x_dir is positive
            let is_pos = y_dir == 1 || x_dir == 1;
            let mut y = if is_pos { map.len() - 1 - y } else { y };
            let mut x = if is_pos { map[y].len() - 1 - x } else { x };

            if map[y][x] == b'O' {
                while (y_dir == -1 && y > 0)
                    || (y_dir == 1 && y < map.len() - 1)
                    || (x_dir == -1 && x > 0)
                    || (x_dir == 1 && x < map[y].len() - 1)
                {
                    let y_off = (y as isize + y_dir) as usize;
                    let x_off = (x as isize + x_dir) as usize;
                    if map[y_off][x_off] == b'.' {
                        map[y_off][x_off] = b'O';
                        map[y][x] = b'.';
                        y = y_off;
                        x = x_off;
                    } else {
                        break;
                    }
                }
            }
        }
    }
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
    roll(&mut map, -1, 0);
    calc_load(&map)
}

fn part_two(input: &str) -> usize {
    let mut map = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();

    fn cycle(map: &mut Vec<Vec<u8>>) {
        roll(map, -1, 0);
        roll(map, 0, -1);
        roll(map, 1, 0);
        roll(map, 0, 1);
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
