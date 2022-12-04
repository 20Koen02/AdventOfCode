use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

type Range = (u32, u32);
type Pairs = Vec<(Range, Range)>;

fn get_pairs(input: &str) -> Pairs {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|pair| {
                    pair.split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn is_subset(range: &Range, other: &Range) -> bool {
    (range.0 >= other.0 && range.1 <= other.1) || (other.0 >= range.0 && other.1 <= range.1)
}

fn is_overlap(range: &Range, other: &Range) -> bool {
    range.0 <= other.1 && range.1 >= other.0
}

fn solve<F>(pairs: &Pairs, f: F) -> u32
where
    F: Fn(&Range, &Range) -> bool,
{
    pairs.iter().map(|(a, b)| f(a, b) as u32).sum()
}

fn main() {
    let pairs = get_pairs(INPUT);
    println!("Day 4 part 1: {}", solve(&pairs, is_subset));
    println!("Day 4 part 2: {}", solve(&pairs, is_overlap));
}
