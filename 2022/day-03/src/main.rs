use helper::solved;

const INPUT: &str = include_str!("in.txt");

fn get_priority(cc: u8) -> u32 {
    (if cc >= 97 { cc - 96 } else { cc - 38 }) as u32
}

fn part1() -> u32 {
    INPUT
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|comp| comp.0.chars().find(|cc| comp.1.contains(*cc)).unwrap() as u8)
        .map(get_priority)
        .sum()
}

fn part2() -> u32 {
    INPUT
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|group| {
            group[0]
                .chars()
                .find(|cc| group[1].contains(*cc) && group[2].contains(*cc))
                .unwrap() as u8
        })
        .map(get_priority)
        .sum()
}

fn main() {
    solved!("Day 3 part 1: {}", part1(), 7553);
    solved!("Day 3 part 2: {}", part2(), 2758);
}
