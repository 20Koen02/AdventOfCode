use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

type Range = (u32, u32);
type Pairs = Vec<(Range, Range)>;

fn get_pairs(input: &str) -> Pairs {
    input
        .lines()
        .map(|line| {
            let vals = line
                .split(['-', ','])
                .map(|v| v.parse::<u32>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();
            ((vals.0, vals.1), (vals.2, vals.3))
        })
        .collect()
}

fn is_subset(range: &Range, other: &Range) -> bool {
    (range.0 >= other.0 && range.1 <= other.1) || (other.0 >= range.0 && other.1 <= range.1)
}

fn is_overlap(range: &Range, other: &Range) -> bool {
    range.0 <= other.1 && range.1 >= other.0
}

fn solve<F>(pairs: &Pairs, f: F) -> usize
where
    F: Fn(&Range, &Range) -> bool,
{
    pairs.iter().filter(|(a, b)| f(a, b)).count()
}

fn main() {
    let pairs = get_pairs(INPUT);
    println!("Day 4 part 1: {}", solve(&pairs, is_subset));
    println!("Day 4 part 2: {}", solve(&pairs, is_overlap));
}
