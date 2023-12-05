use helper::solved;

const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
const INPUT: &str = include_str!("in.txt");

fn solve(s: &str, seed_interpreter: fn(Vec<i64>) -> Vec<(i64, i64)>) -> i64 {
    let groups: Vec<&str> = s.split("\n\n").collect();

    let mut seed_ranges = seed_interpreter(
        groups[0]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect(),
    );

    for g in groups[1..].iter() {
        let step_mapping: Vec<(i64, i64, i64)> = g
            .split('\n')
            .skip(1)
            .map(|l| {
                let nums: Vec<i64> = l.split_whitespace().map(|x| x.parse().unwrap()).collect();
                (nums[0], nums[1], nums[2])
            })
            .collect();

        let mut new_ranges = Vec::new();

        for (mut start, mut r_len) in seed_ranges {
            while r_len != 0 {
                let mut found_match = false;
                let mut best_dist = r_len;

                for &(dst, src, length) in step_mapping.iter() {
                    if src <= start && start < src + length {
                        // Found a match
                        let off = start - src;
                        let rem_length = i64::min(length - off, r_len);
                        new_ranges.push((dst + off, rem_length));
                        start += rem_length;
                        r_len -= rem_length;
                        found_match = true;
                        break;
                    } else if start < src {
                        best_dist = i64::min(src - start, best_dist);
                    }
                }

                if !found_match {
                    let handling_len = i64::min(best_dist, r_len);
                    new_ranges.push((start, handling_len));
                    start += handling_len;
                    r_len -= handling_len;
                }
            }
        }

        seed_ranges = new_ranges;
    }

    seed_ranges.iter().map(|&(start, _)| start).min().unwrap()
}

fn part_one(input: &str) -> i64 {
    fn seed_interpreter(nums: Vec<i64>) -> Vec<(i64, i64)> {
        nums.iter().map(|&n| (n, 1)).collect()
    }

    solve(input, seed_interpreter)
}

fn part_two(input: &str) -> i64 {
    fn seed_interpreter(nums: Vec<i64>) -> Vec<(i64, i64)> {
        nums.chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect()
    }

    solve(input, seed_interpreter)
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 35);
    solved!("Day 5 part 1: {}", part_one(INPUT), 240320250);
    assert_eq!(part_two(TEST_INPUT), 46);
    solved!("Day 5 part 2: {}", part_two(INPUT), 28580589);
}
