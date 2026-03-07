use helper::solved;
use std::collections::VecDeque;
use z3::{Optimize, SatResult, ast::Int};

const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
const INPUT: &str = include_str!("in.txt");

#[derive(Debug, Clone)]
struct Manual {
    indicators: u64,
    buttons: Vec<u64>,
    joltages: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Manual> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let indicators = parts[0]
                .trim_matches(&['[', ']'][..])
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .rev()
                .fold(0, |acc, b| (acc << 1) | b);

            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|s| {
                    s.trim_matches(&['(', ')'][..])
                        .split(',')
                        .filter(|t| !t.is_empty())
                        .map(|n| n.parse::<u64>().unwrap())
                        .fold(0, |acc, b| acc | (1 << b))
                })
                .collect::<Vec<u64>>();

            let joltages = parts[parts.len() - 1]
                .trim_matches(&['{', '}'][..])
                .split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            Manual {
                indicators,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let manuals = parse_input(input);

    manuals
        .iter()
        .map(|m| {
            let num_lights = m.joltages.len();
            let target = m.indicators as usize;

            let states = 1usize << num_lights;

            let mut dist = vec![u16::MAX; states];
            let mut q = VecDeque::new();

            dist[0] = 0;
            q.push_back(0usize);

            while let Some(state) = q.pop_front() {
                if state == target {
                    return dist[state] as u64;
                }

                let d = dist[state];
                for &btn in &m.buttons {
                    let next = state ^ (btn as usize);
                    if dist[next] == u16::MAX {
                        dist[next] = d + 1;
                        q.push_back(next);
                    }
                }
            }

            unreachable!()
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    let manuals = parse_input(input);

    manuals
        .iter()
        .map(|m| {
            let num_buttons = m.buttons.len();

            let opt = Optimize::new();

            let mut vars = Vec::with_capacity(num_buttons);
            for j in 0..num_buttons {
                let v = Int::new_const(format!("x_{}", j));
                opt.assert(&v.ge(&Int::from_i64(0)));
                vars.push(v);
            }

            for (i, &target) in m.joltages.iter().enumerate() {
                let mut affecting: Vec<&Int> = Vec::new();
                for (j, &mask) in m.buttons.iter().enumerate() {
                    if ((mask >> i) & 1) == 1 {
                        affecting.push(&vars[j]);
                    }
                }

                let lhs = if affecting.is_empty() {
                    Int::from_i64(0)
                } else if affecting.len() == 1 {
                    affecting[0].clone()
                } else {
                    Int::add(&affecting)
                };

                opt.assert(&lhs.eq(&Int::from_u64(target)));
            }

            let vars_refs: Vec<&Int> = vars.iter().collect();
            let total = Int::add(&vars_refs);
            opt.minimize(&total);

            match opt.check(&[]) {
                SatResult::Sat => {
                    let model = opt.get_model().unwrap();
                    model.eval(&total, true).unwrap().as_u64().unwrap()
                }
                _ => unreachable!(),
            }
        })
        .sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 7);
    solved!("Day 10 part one {}", part_one(INPUT), 461);

    assert_eq!(part_two(TEST_INPUT), 33);
    solved!("Day 10 part two {}", part_two(INPUT), 16386);
}
