use hashbrown::HashMap;
use itertools::Itertools;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("in.txt");

type State<const N: usize> = ([u8; 11], [[u8; N]; 4]);

fn get_input() -> (&'static [u8], &'static [u8]) {
    INPUT
        .lines()
        .skip(2)
        .take(2)
        .map(|s| s.as_bytes())
        .collect_tuple()
        .unwrap()
}

fn right_configuration<const N: usize>((_, rooms): &State<N>) -> bool {
    rooms
        .iter()
        .zip("ABCD".bytes())
        .all(|(room, c)| room.iter().all(|&x| x == c))
}

fn make_move<const N: usize>(
    (mut corridor, mut rooms): State<N>,
    c: usize,
    room: usize,
    depth: usize,
) -> (usize, State<N>) {
    let piece = if corridor[c] == b'.' {
        rooms[room][depth]
    } else {
        corridor[c]
    } - b'A';
    let c0 = [2, 4, 6, 8][room];
    let energy =
        (depth + if c0 > c { c0 - c } else { c - c0 } + 1) * [1, 10, 100, 1000][piece as usize];
    std::mem::swap(&mut corridor[c], &mut rooms[room][depth]);
    (energy, (corridor, rooms))
}

fn moves<const N: usize>((corridor, rooms): State<N>) -> Vec<(usize, State<N>)> {
    let mut moves = Vec::new();
    for c in 0..corridor.len() {
        // check moving into a room
        if corridor[c] == b'.' {
            continue;
        }
        let room = (corridor[c] - b'A') as usize;
        let c0 = [2, 4, 6, 8][room];
        let (r0, r1) = if c > c0 { (c0, c) } else { (c + 1, c0 + 1) };
        if (r0..r1).any(|i| corridor[i] != b'.') {
            continue;
        }
        let i = match (0..N).take_while(|&i| rooms[room][i] == b'.').last() {
            Some(i) => i,
            _ => continue,
        };
        if (i + 1..N).any(|d| rooms[room][d] != corridor[c]) {
            continue;
        }
        moves.push(make_move((corridor, rooms), c, room, i));
    }
    for room in 0..4 {
        // check moving out of a room
        let i = match (0..N).find(|&i| rooms[room][i] != b'.') {
            Some(i) => i,
            _ => continue,
        };
        let c0 = [2, 4, 6, 8][room];
        let valid_moves = (c0..corridor.len())
            .take_while(|&c| corridor[c] == b'.')
            .chain((0..c0).rev().take_while(|&c| corridor[c] == b'.'))
            .filter(|c| ![2, 4, 6, 8].contains(c))
            .map(|c| make_move((corridor, rooms), c, room, i));
        moves.extend(valid_moves);
    }
    moves
}

fn dijkstra<const N: usize>(state: State<N>) -> i64 {
    let mut dist = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push((0, state));
    while let Some((cost, m)) = q.pop() {
        if right_configuration(&m) {
            return -cost;
        }
        if let Some(&c) = dist.get(&m) {
            if -cost > c {
                continue;
            }
        }
        for (energy, m) in moves(m) {
            let next_cost = -cost + energy as i64;
            let &c = dist.get(&m).unwrap_or(&1000000);
            if c > next_cost {
                dist.insert(m, next_cost);
                q.push((-next_cost, m));
            }
        }
    }
    unreachable!()
}

fn main() {
    let (l1, l2) = get_input();
    println!(
        "Day 23 part 1: {}",
        dijkstra((
            [b'.'; 11],
            [
                [l1[3], l2[3]],
                [l1[5], l2[5]],
                [l1[7], l2[7]],
                [l1[9], l2[9]],
            ],
        ))
    );
    println!(
        "Day 23 part 2: {}",
        dijkstra((
            [b'.'; 11],
            [
                [l1[3], b'D', b'D', l2[3]],
                [l1[5], b'C', b'B', l2[5]],
                [l1[7], b'B', b'A', l2[7]],
                [l1[9], b'A', b'C', l2[9]],
            ],
        ))
    );
}
