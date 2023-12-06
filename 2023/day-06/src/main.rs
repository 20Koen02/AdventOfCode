use helper::solved;

const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
const INPUT: &str = include_str!("in.txt");

fn get_nums_part_one(input: &str, row: usize) -> Vec<usize> {
    let (_, num) = input.lines().nth(row).unwrap().split_once(' ').unwrap();
    num.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn get_nums_part_two(input: &str, row: usize) -> Vec<usize> {
    let (_, num) = input.lines().nth(row).unwrap().split_once(' ').unwrap();
    vec![num.replace(' ', "").parse().unwrap()]
}

fn solve(times: Vec<usize>, records: Vec<usize>) -> usize {
    times.iter().enumerate().fold(1, |acc, (idx, time)| {
        acc * (1..*time).fold(0, |acc, i| {
            if (time - i) * i > records[idx] {
                acc + 1
            } else {
                acc
            }
        })
    })
}

fn part_one(input: &str) -> usize {
    let times = get_nums_part_one(input, 0);
    let records = get_nums_part_one(input, 1);
    solve(times, records)
}

fn part_two(input: &str) -> usize {
    let times = get_nums_part_two(input, 0);
    let records = get_nums_part_two(input, 1);
    solve(times, records)
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 288);
    solved!("Day 6 part one: {}", part_one(INPUT), 140220);
    assert_eq!(part_two(TEST_INPUT), 71503);
    solved!("Day 6 part two: {}", part_two(INPUT), 39570185);
}
