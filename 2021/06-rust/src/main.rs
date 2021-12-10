const INPUT: &str = include_str!("in.txt");

type FishCount = [usize; 9];

fn get_fish_count() -> FishCount {
    return INPUT
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .fold([0usize; 9], |mut acc, val| {
            acc[val] += 1;
            acc
        });
}

fn simulate(mut fishes: FishCount, rounds: usize) -> usize {
    for _ in 0..rounds {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }
    fishes.iter().sum()
}

fn main() {
    println!("Day 6 part one: {}", simulate(get_fish_count(), 80));
    println!("Day 6 part two: {}", simulate(get_fish_count(), 256));
}
