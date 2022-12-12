use std::collections::VecDeque;

use helper::solved;

const INPUT: &str = include_str!("in.txt");

type HeightMap = Vec<Vec<u8>>;
type PT = (usize, usize);

struct QueueNode {
    pt: PT,
    dist: usize,
}

fn get_hm() -> (HeightMap, PT, PT, Vec<PT>) {
    let mut source: PT = (0, 0);
    let mut target: PT = (0, 0);
    let mut source_pts: Vec<PT> = vec![];

    let hm: HeightMap = INPUT
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.bytes()
                .enumerate()
                .map(|(j, c)| match c {
                    b'S' => {
                        source = (i, j);
                        source_pts.push((i, j));
                        b'a'
                    }
                    b'E' => {
                        target = (i, j);
                        b'z'
                    }
                    b'a' => {
                        source_pts.push((i, j));
                        b'a'
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();

    (hm, source, target, source_pts)
}

fn is_valid(row: isize, col: isize, hm: &HeightMap) -> bool {
    row >= 0 && row < hm.len() as isize && col >= 0 && col < hm[0].len() as isize
}

fn solve(hm: &HeightMap, source: PT, target: PT) -> Option<usize> {
    let mut visisted: Vec<Vec<bool>> = vec![vec![false; hm[0].len()]; hm.len()];
    visisted[source.0][source.1] = true;
    let mut q: VecDeque<QueueNode> = VecDeque::from(vec![QueueNode {
        pt: source,
        dist: 0,
    }]);

    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        if curr.pt == target {
            return Some(curr.dist);
        }

        for (r, c) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let row = curr.pt.0 as isize + r;
            let col = curr.pt.1 as isize + c;

            if is_valid(row, col, hm)
                && hm[curr.pt.0][curr.pt.1] + 1 >= hm[row as usize][col as usize]
                && !visisted[row as usize][col as usize]
            {
                visisted[row as usize][col as usize] = true;
                q.push_back(QueueNode {
                    pt: (row as usize, col as usize),
                    dist: curr.dist + 1,
                });
            }
        }
    }
    None
}

fn main() {
    let (hm, source, target, source_pts) = get_hm();

    let part1 = solve(&hm, source, target).unwrap();
    solved!("Day 12 part 1: {}", part1, 391);

    let part2 = source_pts
        .iter()
        .filter_map(|&src| solve(&hm, src, target))
        .min()
        .unwrap();
    solved!("Day 12 part 2: {}", part2, 386);
}
