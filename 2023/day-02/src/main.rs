use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
const INPUT: &str = include_str!("in.txt");

fn solve(input: &str) -> (u32, u32) {
    let mut part_one: u32 = 0;
    let mut part_two: u32 = 0;

    for (i, line) in input.trim().split('\n').enumerate() {
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
    let (part_one, part_two) = solve(INPUT);
    let (test_one, test_two) = solve(TEST_INPUT);

    assert_eq!(test_one, 8);
    solved!("Day 2 part one: {}", part_one, 2416);
    assert_eq!(test_two, 2286);
    solved!("Day 2 part two: {}", part_two, 63307);
}
