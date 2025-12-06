use helper::solved;

const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
const INPUT: &str = include_str!("in.txt");

fn parse_input(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.trim().lines().map(|line: &str| {
        let (dir, value) = line.split_at(1);
        let n: i32 = value.parse().unwrap();

        match dir {
            "L" => -n,
            "R" => n,
            _ => unreachable!(),
        }
    })
}

fn part_one(input: &str) -> u32 {
    let mut hit_zero = 0;
    let mut pos = 50;

    for step in parse_input(input) {
        pos = (pos + step).rem_euclid(100);

        if pos == 0 {
            hit_zero += 1;
        }
    }

    hit_zero
}

fn part_two(input: &str) -> u32 {
    let mut hit_zero = 0u32;
    let mut pos = 50i32;

    for step in parse_input(input) {
        let new_pos = pos + step;

        if new_pos < 1 && pos != 0 {
            hit_zero += 1;
        }

        if new_pos < 1 || new_pos > 99 {
            hit_zero += (new_pos / 100).abs() as u32;
        }

        pos = new_pos.rem_euclid(100);
    }

    hit_zero
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 3);
    solved!("Day 1 part one {}", part_one(INPUT), 1026);

    assert_eq!(part_two(TEST_INPUT), 6);
    solved!("Day 1 part two {}", part_two(INPUT), 5923);
}
