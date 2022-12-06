use helper::solved;
use itertools::Itertools;

const INPUT: &str = include_str!("in.txt");

/// Vec of calories sorted from highest to lowest
fn sorted_elves() -> Vec<u32> {
    INPUT
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .sorted()
        .rev()
        .collect::<Vec<u32>>()
}

fn main() {
    let cal_totals = sorted_elves();
    solved!("Day 1 part one: {}", cal_totals[0], 70509);
    solved!(
        "Day 1 part two: {}",
        cal_totals.iter().take(3).sum::<u32>(),
        208567
    );
}
