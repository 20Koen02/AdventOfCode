use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = include_str!("test_in.txt");
const INPUT: &str = include_str!("in.txt");

type PinHeights = [isize; 5];

fn parse(input: &str) -> (Vec<PinHeights>, Vec<PinHeights>) {
    let schematics = input.split("\n\n").collect::<Vec<_>>();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in schematics {
        let lines = schematic.lines().collect::<Vec<_>>();
        let mut pin_heights = [-1; 5];

        for line in &lines {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    pin_heights[i] += 1;
                }
            }
        }

        if lines[0].contains("#") {
            locks.push(pin_heights);
        } else {
            keys.push(pin_heights);
        }
    }

    (locks, keys)
}

fn part_one(input: &str) -> usize {
    let (locks, keys) = parse(input);

    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(&lock, &key)| lock.iter().zip(key).all(|(l, k)| l + k <= 5))
        .count()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 3);
    solved!("Day 24 part one: {}", part_one(INPUT), 3663);
}
