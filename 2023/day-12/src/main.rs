use cached::proc_macro::cached;
use helper::solved;
use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

#[cached]
fn get_count(
    springs: String,
    ecc: Vec<usize>,
    next_char: usize,
    current_count: usize,
    hash_seqs: usize,
) -> usize {
    if next_char == springs.len() {
        (ecc.len() == hash_seqs) as usize
    } else if springs.chars().nth(next_char).unwrap() == '#' {
        get_count(springs, ecc, next_char + 1, current_count + 1, hash_seqs)
    } else if springs.chars().nth(next_char).unwrap() == '.' || hash_seqs == ecc.len() {
        if hash_seqs < ecc.len() && current_count == ecc[hash_seqs] {
            get_count(springs, ecc, next_char + 1, 0, hash_seqs + 1)
        } else if current_count == 0 {
            get_count(springs, ecc, next_char + 1, 0, hash_seqs)
        } else {
            0
        }
    } else {
        let hash_count = get_count(
            springs.clone(),
            ecc.clone(),
            next_char + 1,
            current_count + 1,
            hash_seqs,
        );

        let dot_count = if current_count == ecc[hash_seqs] {
            get_count(springs, ecc, next_char + 1, 0, hash_seqs + 1)
        } else if current_count == 0 {
            get_count(springs, ecc, next_char + 1, 0, hash_seqs)
        } else {
            0
        };

        hash_count + dot_count
    }
}

fn part_one(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (springs, ecc) = line.split_once(' ').unwrap();
        let springs = springs.to_owned() + ".";
        let ecc = ecc
            .split(',')
            .map(|count| count.parse().unwrap())
            .collect_vec();

        acc + get_count(springs, ecc, 0, 0, 0)
    })
}

fn part_two(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (springs, ecc) = line.split_once(' ').unwrap();

        let springs = std::iter::repeat(springs.to_owned())
            .take(5)
            .collect_vec()
            .join("?")
            + ".";

        let ecc = ecc
            .split(',')
            .map(|count| count.parse().unwrap())
            .collect_vec()
            .repeat(5);

        acc + get_count(springs, ecc, 0, 0, 0)
    })
}

fn main() {
    solved!("Day 12 part 1: {}", part_one(INPUT), 7007);
    solved!("Day 12 part 2: {}", part_two(INPUT), 3476169006222_usize);
}
