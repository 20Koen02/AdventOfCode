use helper::solved;

const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
const INPUT: &str = include_str!("in.txt");

fn count_xmas(rows: &Vec<String>) -> u32 {
    rows.iter()
        .map(|s| (s.matches("XMAS").count() + s.matches("SAMX").count()) as u32)
        .sum::<u32>()
}

fn columns(rows: &Vec<String>) -> Vec<String> {
    (0..rows[0].len())
        .map(|i| rows.iter().map(|row| row.chars().nth(i).unwrap()).collect())
        .collect()
}

fn diagonals(rows: &Vec<String>) -> Vec<String> {
    (0..(rows.len() + rows[0].len() - 1))
        .map(|d| {
            let mut diagonal = String::new();
            for (i, row) in rows.iter().enumerate() {
                let j = d as isize - i as isize;
                if j >= 0 && (j as usize) < row.len() {
                    diagonal.push(row.chars().nth(j as usize).unwrap());
                }
            }
            diagonal
        })
        .collect()
}

fn mirror(rows: &Vec<String>) -> Vec<String> {
    rows.iter().map(|row| row.chars().rev().collect()).collect()
}

fn part_one(input: &str) -> u32 {
    let rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    count_xmas(&rows)
        + count_xmas(&columns(&rows))
        + count_xmas(&diagonals(&rows))
        + count_xmas(&diagonals(&mirror(&rows)))
}

fn part_two(input: &str) -> u32 {
    let rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let size = rows.len();
    let mut count = 0;

    for y in 1..size - 1 {
        for x in 1..rows[y].len() - 1 {
            if rows[y].chars().nth(x) == Some('A') {
                let top_left = rows[y - 1].chars().nth(x - 1);
                let bottom_right = rows[y + 1].chars().nth(x + 1);
                let top_right = rows[y - 1].chars().nth(x + 1);
                let bottom_left = rows[y + 1].chars().nth(x - 1);

                if matches!(
                    (top_left.zip(bottom_right), top_right.zip(bottom_left)),
                    (
                        Some(('M', 'S')) | Some(('S', 'M')),
                        Some(('M', 'S')) | Some(('S', 'M'))
                    )
                ) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 18);
    solved!("Day 3 part one: {}", part_one(INPUT), 2344);

    assert_eq!(part_two(TEST_INPUT), 9);
    solved!("Day 3 part two: {}", part_two(INPUT), 1815);
}
