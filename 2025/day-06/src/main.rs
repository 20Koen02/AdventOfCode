use helper::solved;

const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
const INPUT: &str = include_str!("in.txt");

fn part_one(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let ops: Vec<u8> = lines
        .iter()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.as_bytes()[0])
        .collect();

    let nums_matrix: Vec<Vec<u64>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    ops.iter().enumerate().fold(0, |acc, (col, op)| {
        acc + nums_matrix.iter().map(|row| row[col]).fold(
            if *op == b'*' { 1 } else { 0 },
            |acc, n| {
                if *op == b'*' { acc * n } else { acc + n }
            },
        )
    })
}

fn part_two(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let ops: Vec<(usize, u8)> = lines
        .last()
        .unwrap()
        .bytes()
        .enumerate()
        .filter(|(_, b)| *b != b' ')
        .map(|(i, b)| (i, b))
        .collect();

    ops.iter().fold(0, |acc, (si, op)| {
        let mut nums_bytes: Vec<Vec<u8>> = vec![];
        lines.iter().take(lines.len() - 1).for_each(|line| {
            let mut consumed_char = false;
            for (ci, c) in line.bytes().skip(*si).enumerate() {
                if c == b' ' {
                    if consumed_char {
                        break;
                    } else {
                        continue;
                    }
                }
                consumed_char = true;

                while nums_bytes.len() <= ci {
                    nums_bytes.push(vec![]);
                }
                nums_bytes[ci].push(c - b'0');
            }
        });

        acc + nums_bytes
            .iter()
            .fold(if *op == b'*' { 1 } else { 0 }, |acc, digits| {
                let val: u64 = digits.iter().fold(0, |acc, d| acc * 10 + *d as u64);
                if *op == b'*' { acc * val } else { acc + val }
            })
    })
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 4277556);
    solved!("Day 6 part one {}", part_one(INPUT), 8108520669952u64);

    assert_eq!(part_two(TEST_INPUT), 3263827);
    solved!("Day 6 part two {}", part_two(INPUT), 11708563470209u64);
}
