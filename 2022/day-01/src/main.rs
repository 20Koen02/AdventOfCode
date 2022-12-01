use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

/// Vec of calories sorted from highest to lowest
fn sorted_elves() -> Vec<usize> {
    INPUT
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines().map(|c| c.parse::<usize>().unwrap()).sum())
        .sorted()
        .rev()
        .collect::<Vec<usize>>()
}

fn main() {
    let cal_totals = sorted_elves();
    println!("Day 1 part one: {}", cal_totals[0]);
    println!(
        "Day 1 part two: {}",
        cal_totals.iter().take(3).sum::<usize>()
    );
}
