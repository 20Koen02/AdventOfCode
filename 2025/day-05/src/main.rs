use helper::solved;

const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
const INPUT: &str = include_str!("in.txt");

fn parse_ingredients(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, available) = input.split_once("\n\n").unwrap();

    let fresh_range = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let available = available
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (fresh_range, available)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_unstable_by_key(|r| r.0);
    let mut write = 0;

    for i in 1..ranges.len() {
        let (s, e) = ranges[i];
        let (_, cur_e) = ranges[write];

        if s <= cur_e + 1 {
            if e > cur_e {
                ranges[write].1 = e;
            }
        } else {
            write += 1;
            ranges[write] = (s, e);
        }
    }

    ranges.truncate(write + 1);
    ranges
}

fn part_one(input: &str) -> u64 {
    let (ranges, mut available) = parse_ingredients(input);
    let merged = merge_ranges(ranges);
    available.sort_unstable();

    let mut count = 0;
    let mut r_idx = 0;

    for item in available {
        while r_idx < merged.len() && item > merged[r_idx].1 {
            r_idx += 1;
        }
        if r_idx == merged.len() {
            break;
        }

        let (start, end) = merged[r_idx];
        if item >= start && item <= end {
            count += 1;
        }
    }

    count
}

fn part_two(input: &str) -> u64 {
    let (ranges, _) = parse_ingredients(input);
    let merged = merge_ranges(ranges);

    merged.into_iter().map(|(s, e)| e - s + 1).sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 3);
    solved!("Day 5 part one {}", part_one(INPUT), 775);

    assert_eq!(part_two(TEST_INPUT), 14);
    solved!("Day 5 part two {}", part_two(INPUT), 350684792662845u64);
}
