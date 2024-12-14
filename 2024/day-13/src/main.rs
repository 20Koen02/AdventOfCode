use helper::solved;

const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
const INPUT: &str = include_str!("in.txt");

type ClawMachine = [isize; 6];

fn parse(input: &str, part_two: bool) -> Vec<ClawMachine> {
    input
        .trim()
        .split("\n\n")
        .map(|s| {
            let re = regex::Regex::new(r"\d+").unwrap();
            let mut numbers: Vec<isize> = re
                .find_iter(s)
                .map(|m| m.as_str().parse().unwrap())
                .collect();

            if part_two {
                numbers[4] += 10000000000000;
                numbers[5] += 10000000000000;
            }

            assert_eq!(numbers.len(), 6);
            numbers.as_slice().try_into().unwrap()
        })
        .collect()
}

fn solve(input: &str, part_two: bool) -> usize {
    let machines = parse(input, part_two);

    machines
        .iter()
        .map(|m| {
            let [a_x, a_y, b_x, b_y, p_x, p_y] = m;

            let denom = a_x * b_y - a_y * b_x;
            if denom == 0 {
                return 0;
            }

            let numerator_a = p_x * b_y - p_y * b_x;
            let numerator_b = a_x * p_y - a_y * p_x;

            if numerator_a % denom != 0 || numerator_b % denom != 0 {
                return 0;
            }

            let a = numerator_a / denom;
            let b = numerator_b / denom;

            (a * 3 + b) as usize
        })
        .sum()
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 480);
    solved!("Day 13 part one: {}", solve(INPUT, false), 37686);

    assert_eq!(solve(TEST_INPUT, true), 875318608908);
    solved!(
        "Day 13 part two: {}",
        solve(INPUT, true),
        77204516023437usize
    );
}
