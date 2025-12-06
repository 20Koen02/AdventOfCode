use helper::solved;
use itoa::Buffer;

const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const INPUT: &str = include_str!("in.txt");

fn parse_ids(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .trim()
        .split(',')
        .map(|range: &str| {
            let (s, e) = range.split_once('-').unwrap();
            (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())
        })
        .flat_map(|(start, end)| start..=end)
}

fn part_one(input: &str) -> u64 {
    let mut buf = Buffer::new();

    parse_ids(input)
        .filter(|&n| {
            let s = buf.format(n).as_bytes();
            let len = s.len();
            if len % 2 != 0 {
                return false;
            }
            let half = len / 2;
            s[..half] == s[half..]
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    let mut buf = Buffer::new();

    parse_ids(input)
        .filter(|&n| {
            let s = buf.format(n).as_bytes();
            let len = s.len();

            'chunks: for d in 1..=len / 2 {
                if len % d != 0 {
                    continue;
                }
                let pattern = &s[..d];
                let mut i = d;
                while i < len {
                    if &s[i..i + d] != pattern {
                        continue 'chunks;
                    }
                    i += d;
                }
                return true;
            }
            false
        })
        .sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 1227775554);
    solved!("Day 2 part one {}", part_one(INPUT), 38310256125u64);

    assert_eq!(part_two(TEST_INPUT), 4174379265);
    solved!("Day 2 part two {}", part_two(INPUT), 58961152806u64);
}
