use helper::solved;

fn solve(times: Vec<u64>, records: Vec<u64>) -> usize {
    times.iter().enumerate().fold(1, |acc, (idx, time)| {
        acc * (1..*time).fold(0, |acc, i| {
            if (time - i) * i > records[idx] {
                acc + 1
            } else {
                acc
            }
        })
    })
}

fn main() {
    assert_eq!(solve(vec![7, 15, 30], vec![9, 40, 200]), 288);
    solved!(
        "Day 6 part one: {}",
        solve(vec![53, 83, 72, 88], vec![333, 1635, 1289, 1532]),
        140220
    );
    assert_eq!(solve(vec![71530], vec![940200]), 71503);
    solved!(
        "Day 6 part two: {}",
        solve(vec![53837288], vec![333163512891532]),
        39570185
    );
}
