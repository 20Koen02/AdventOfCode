use helper::solved;

const TEST_INPUT: &str = include_str!("in_test.txt");
const INPUT: &str = include_str!("in.txt");

type Rule = (u32, u32);
type Update = Vec<u32>;

fn parse(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules: Vec<Rule> = rules_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let updates: Vec<Update> = updates_str
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn get_correct(rules: &Vec<Rule>, updates: &Vec<Update>) -> (Vec<Update>, Vec<Update>) {
    let mut correct = vec![];
    let mut incorrect = vec![];

    for update in updates {
        let corr = rules.iter().all(|(x, y)| {
            if !update.contains(x) || !update.contains(y) {
                return true;
            }

            update.iter().position(|&i| i == *x).unwrap()
                < update.iter().position(|&i| i == *y).unwrap()
        });
        if corr {
            correct.push(update.to_vec());
        } else {
            incorrect.push(update.to_vec());
        }
    }

    (correct, incorrect)
}

fn part_one(input: &str) -> u32 {
    let (rules, updates) = parse(input);

    get_correct(&rules, &updates)
        .0
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_two(input: &str) -> u32 {
    let (rules, updates) = parse(input);

    let mut incorrect = get_correct(&rules, &updates).1;

    for update in incorrect.iter_mut() {
        let mut prev = vec![];
        while prev != *update {
            prev = update.clone();
            for (x, y) in rules.iter() {
                if !update.contains(x) || !update.contains(y) {
                    continue;
                }

                if update.iter().position(|&i| i == *x).unwrap()
                    > update.iter().position(|&i| i == *y).unwrap()
                {
                    let pos_x = update.iter().position(|&i| i == *x).unwrap();
                    let pos_y = update.iter().position(|&i| i == *y).unwrap();
                    update.swap(pos_x, pos_y);
                }
            }
        }
    }

    incorrect
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 143);
    solved!("Day 5 part one: {}", part_one(INPUT), 5391);

    assert_eq!(part_two(TEST_INPUT), 123);
    solved!("Day 5 part two: {}", part_two(INPUT), 6142);
}
