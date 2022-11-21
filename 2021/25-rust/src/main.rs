const INPUT: &str = include_str!("in.txt");

fn get_initial_state() -> Vec<Vec<char>> {
    return INPUT
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
}

fn solve(matrix: Vec<Vec<char>>) -> usize {
    let mut matrix = matrix;

    let n = matrix.len();
    let m = matrix[0].len();

    for step in 1.. {
        let mut moved = false;
        // first east then south
        for (f, (x, y)) in [('>', (0, 1)), ('v', (1, 0))] {
            let mut next = matrix.clone();
            // go through all the cells to move the sea cucumbers and track if any moved
            for i in 0..n {
                for j in 0..m {
                    if matrix[i][j] == f && matrix[(i + x) % n][(j + y) % m] == '.' {
                        next[(i + x) % n][(j + y) % m] = f;
                        next[i][j] = '.';
                        moved = true;
                    }
                }
            }
            matrix = next;
        }
        // if no sea cucumbers moved, we are done (like game of life)
        if !moved {
            return step;
        }
    }
    return 0;
}

fn main() {
    let init = get_initial_state();
    println!("Day 25: {}", solve(init));
}
