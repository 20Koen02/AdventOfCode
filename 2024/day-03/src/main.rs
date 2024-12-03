use helper::solved;
use regex::Regex;

const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_INPUT_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
const INPUT: &str = include_str!("in.txt");

fn part_one(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<u32>().unwrap();
            let y = cap[2].parse::<u32>().unwrap();
            x * y
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();

    let mut enabled = true;
    re.captures_iter(input)
        .map(|cap| {
            let instruction = cap[0].to_string();
            if instruction == "do()" {
                enabled = true;
            } else if instruction == "don't()" {
                enabled = false;
            } else if enabled {
                let x = cap[1].parse::<u32>().unwrap();
                let y = cap[2].parse::<u32>().unwrap();
                return x * y;
            }
            0
        })
        .sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 161);
    solved!("Day 3 part one: {}", part_one(INPUT), 173785482);

    assert_eq!(part_two(TEST_INPUT_2), 48);
    solved!("Day 3 part two: {}", part_two(INPUT), 83158140);
}
