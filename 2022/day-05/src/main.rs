const INPUT: &str = include_str!("in.txt");

type Move = (usize, usize, usize);
type Moves = Vec<Move>;
type Stacks = Vec<Vec<char>>;

fn parse_input() -> (Stacks, Moves) {
    let (stacks_str, moves_str) = INPUT.split_once("\n\n").unwrap();

    let mut stacks: Stacks = vec![vec![]; (stacks_str.lines().last().unwrap().len() + 1) / 4];
    stacks_str.lines().rev().skip(1).for_each(|line| {
        line.chars().skip(1).enumerate().for_each(|(i, c)| {
            if c.is_ascii_uppercase() {
                stacks[i / 4].push(c);
            }
        })
    });

    let mut moves: Moves = vec![];
    moves_str.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        moves.push((
            parts.nth(1).unwrap().parse().unwrap(),
            parts.nth(1).unwrap().parse().unwrap(),
            parts.nth(1).unwrap().parse().unwrap(),
        ));
    });

    (stacks, moves)
}

fn calc_top(stacks: &Stacks) -> String {
    stacks.iter().fold(String::new(), |acc: String, stack| {
        acc + &stack.last().unwrap().to_string()
    })
}

fn solve(stacks: &Stacks, moves: &Moves, single_crate_pickup: bool) -> String {
    let mut stacks: Stacks = stacks.clone();
    moves.iter().for_each(|(count, from, to)| {
        let drain_idx = stacks[*from - 1].len() - count;
        let mut moving: Vec<char> = stacks[*from - 1].drain(drain_idx..).collect();
        if single_crate_pickup {
            moving.reverse();
        }
        stacks[*to - 1].append(&mut moving);
    });
    calc_top(&stacks)
}

fn main() {
    let (stacks, moves) = parse_input();
    println!("Day 5 part 1: {}", solve(&stacks, &moves, true));
    println!("Day 5 part 2: {}", solve(&stacks, &moves, false));
}
