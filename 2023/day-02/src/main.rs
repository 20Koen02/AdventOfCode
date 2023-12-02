use helper::solved;
use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

fn solve() -> (u32, u32) {
    let mut part_one: u32 = 0;
    let mut part_two: u32 = 0;

    for (i, line) in INPUT.trim().split('\n').enumerate() {
        let cubes = line.split(": ").nth(1).unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut impossible = false;

        for cube in cubes.split([';', ',']) {
            let (num, colour) = cube.trim().split(' ').collect_tuple().unwrap();
            let num = num.parse::<u32>().unwrap();

            impossible = match colour {
                "red" => {
                    red = num.max(red);
                    num > 12
                }
                "green" => {
                    green = num.max(green);
                    num > 13
                }
                "blue" => {
                    blue = num.max(blue);
                    num > 14
                }
                _ => unreachable!(),
            } || impossible;
        }

        if !impossible {
            part_one += i as u32 + 1;
        }
        part_two += red * green * blue
    }

    (part_one, part_two)
}

fn main() {
    let (part_one, part_two) = solve();
    solved!("Day 2 part one: {}", part_one, 2416);
    solved!("Day 2 part two: {}", part_two, 63307);
}