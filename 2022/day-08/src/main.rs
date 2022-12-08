use helper::solved;
use std::vec;

const INPUT: &str = include_str!("in.txt");

type Matrix = Vec<Vec<usize>>;

fn get_matrix() -> Matrix {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn get_neighbors(matrix: &Matrix, x: usize, y: usize) -> Vec<Vec<usize>> {
    vec![
        matrix[y][0..x]
            .iter()
            .copied()
            .rev()
            .collect::<Vec<usize>>(),
        matrix[y][x + 1..].to_vec(),
        matrix[0..y]
            .iter()
            .map(|row| &row[x])
            .copied()
            .rev()
            .collect::<Vec<usize>>(),
        matrix[y + 1..]
            .iter()
            .map(|row| &row[x])
            .copied()
            .collect::<Vec<usize>>(),
    ]
}

fn is_visible(matrix: &Matrix, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || y == matrix.len() - 1 || x == matrix[y].len() - 1 {
        return true;
    }

    get_neighbors(matrix, x, y)
        .iter()
        .map(|n| n.iter().max().unwrap())
        .min()
        .unwrap()
        < &matrix[y][x]
}

fn scenic_score(matrix: &Matrix, x: usize, y: usize) -> usize {
    let height = matrix[y][x];
    get_neighbors(matrix, x, y)
        .iter()
        .map(|neighbor| {
            let mut n: usize = 0;
            for k in neighbor {
                n += 1;
                if k >= &height {
                    break;
                }
            }
            n
        })
        .product()
}

fn part1(matrix: &Matrix) -> usize {
    matrix.iter().enumerate().fold(0, |mut acc, (y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            acc += is_visible(matrix, x, y) as usize;
        });
        acc
    })
}

fn part2(matrix: &Matrix) -> usize {
    matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| scenic_score(matrix, x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let matrix = get_matrix();
    solved!("Day 8 part 1: {}", part1(&matrix), 1684);
    solved!("Day 8 part 2: {}", part2(&matrix), 486540);
}
