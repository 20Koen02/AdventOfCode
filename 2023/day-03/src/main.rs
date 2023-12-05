use std::collections::HashMap;

use helper::solved;

const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
const INPUT: &str = include_str!("in.txt");

fn add_matrix_padding(matrix: &mut Vec<Vec<char>>) {
    matrix.iter_mut().for_each(|line| {
        line.insert(0, '.');
        line.push('.');
    });
    let dots = vec!['.'; matrix[0].len()];
    matrix.insert(0, dots.clone());
    matrix.push(dots);
}

fn get_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    add_matrix_padding(&mut matrix);

    matrix
}

fn get_number_at_index(index: usize, line: &[char]) -> Option<u32> {
    let mut number_chars: Vec<char> = Vec::new();
    for &char_at_index in line.get(index..)? {
        match char_at_index {
            '0'..='9' => {
                number_chars.push(char_at_index);
            }
            _ => break,
        }
    }
    number_chars.iter().collect::<String>().parse().ok()
}

fn get_neighbours(
    x: usize,
    xlen: usize,
    y: usize,
    matrix: &[Vec<char>],
) -> Vec<(char, usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    // aaaaa
    // c123d
    // bbbbb
    for i in 0..=xlen + 1 {
        neighbours.push((y - 1, x - 1 + i)); // a
        neighbours.push((y + 1, x - 1 + i)); // b
    }
    neighbours.push((y, x - 1)); // c
    neighbours.push((y, x + xlen)); // d

    neighbours
        .iter()
        .map(|(y, x)| (matrix[*y][*x], *y, *x))
        .collect()
}

fn solve(input: &str) -> (u32, u32) {
    let matrix = get_matrix(input);

    let mut current_number: u32 = 0;
    let mut part_number: u32 = 0;

    let mut gears = HashMap::new();

    matrix.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            match c {
                '0'..='9' => {
                    if current_number == 0 {
                        current_number = get_number_at_index(x, line).unwrap();
                        let number_len = current_number.to_string().len();
                        let neighbours = get_neighbours(x, number_len, y, &matrix);

                        if !neighbours
                            .iter()
                            .all(|(c, _, _)| c.is_ascii_digit() || *c == '.')
                        {
                            part_number += current_number;
                        }

                        neighbours
                            .iter()
                            .filter_map(|(_, y, x)| match matrix[*y][*x] {
                                '0'..='9' | '.' => None,
                                _ => Some((*y, *x)),
                            })
                            .for_each(|loc| {
                                gears.entry(loc).or_insert(vec![]).push(current_number);
                            });
                    }
                }
                _ => current_number = 0,
            };
        });
    });

    let gear_total = gears.iter().fold(
        0,
        |acc, (_, v)| {
            if v.len() == 2 {
                acc + v[0] * v[1]
            } else {
                acc
            }
        },
    );

    (part_number, gear_total)
}

fn main() {
    let (part_one, part_two) = solve(INPUT);
    let (test_one, test_two) = solve(TEST_INPUT);

    assert_eq!(test_one, 4361);
    solved!("Day 3 part one: {}", part_one, 539433);
    assert_eq!(test_two, 467835);
    solved!("Day 3 part two: {}", part_two, 75847567);
}
