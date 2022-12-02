const INPUT: &str = include_str!("in.txt");

type Strat = Vec<(usize, usize)>;

fn get_strat() -> Strat {
    INPUT
        .lines()
        .map(|l| {
            let moves: Vec<u8> = l
                .split(" ")
                .map(|m| m.chars().nth(0).unwrap() as u8)
                .collect();
            (moves[0] as usize - 65, moves[1] as usize - 88) // ascii to 0, 1, 2
        })
        .collect()
}

fn part1(strat: Strat) -> usize {
    strat.iter().fold(0, |mut acc, (p2, p1)| {
        if p1 == p2 {
            acc += 3
        } else if (p1 + 2) % 3 == *p2 {
            acc += 6
        }
        acc + p1 + 1
    })
}

fn part2(strat: Strat) -> usize {
    strat.iter().fold(0, |mut acc, (p2, p1)| {
        match p1 {
            0 => acc += (p2 + 2) % 3 + 1,
            1 => acc += p2 + 1,
            2 => acc += (p2 + 1) % 3 + 1,
            _ => unreachable!(),
        }
        acc + p1 * 3
    })
}

fn main() {
    let strat = get_strat();
    println!("Day 2 part 1: {}", part1(strat.clone()));
    println!("Day 2 part 2: {}", part2(strat.clone()));
}
