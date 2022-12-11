use helper::solved;
use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

enum Operation {
    Add(usize),
    Mul(usize),
    MulSelf,
}

impl Operation {
    fn execute(&self, item: usize) -> usize {
        match self {
            Operation::Add(val) => item + val,
            Operation::Mul(val) => item * val,
            Operation::MulSelf => item * item,
        }
    }
}

struct Monkey {
    inspects: usize,
    items: Vec<usize>,
    operation: Operation,
    test_div: usize,
    decision: [usize; 2],
}

impl Monkey {
    fn from_str(m: &str) -> Monkey {
        let mut m_lines = m.lines();

        let items = m_lines.nth(1).unwrap()[18..]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let operation = match m_lines.next().unwrap()[23..].split_once(' ').unwrap() {
            ("+", val) => Operation::Add(val.parse().unwrap()),
            ("*", "old") => Operation::MulSelf,
            ("*", val) => Operation::Mul(val.parse().unwrap()),
            _ => unreachable!(),
        };

        let (test_div, if_true, if_false) = m_lines
            .map(|l| l.split(' ').last().unwrap().parse().unwrap())
            .collect_tuple()
            .unwrap();

        Monkey {
            inspects: 0,
            items,
            operation,
            test_div,
            decision: [if_false, if_true],
        }
    }
}

fn get_monkeys() -> Vec<Monkey> {
    INPUT.split("\n\n").map(Monkey::from_str).collect()
}

fn solve(rounds: usize, div: usize) -> usize {
    let mut monkeys = get_monkeys();
    let mod_product = monkeys.iter().map(|x| x.test_div).product::<usize>();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                monkeys[i].inspects += 1;
                let item = monkeys[i].items.pop().unwrap();

                let worry = monkeys[i].operation.execute(item) / div % mod_product;
                let throw_to = monkeys[i].decision[(worry % monkeys[i].test_div == 0) as usize];
                monkeys[throw_to].items.push(worry);
            }
        }
    }

    let mut inspects = monkeys.iter().map(|x| x.inspects).collect::<Vec<_>>();
    inspects.sort_by(|a, b| b.cmp(a));
    inspects[0] * inspects[1]
}

fn main() {
    solved!("Day 11 part 1: {}", solve(20, 3), 56350_usize);
    solved!("Day 11 part 2: {}", solve(10_000, 1), 13954061248_usize);
}
