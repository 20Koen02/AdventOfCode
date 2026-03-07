use helper::solved;
use itertools::Itertools;

const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
const INPUT: &str = include_str!("in.txt");

type Point = (u64, u64, u64);

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            line.splitn(3, ',')
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn get_connections(coords: &[Point]) -> Vec<(u64, usize, usize)> {
    let n = coords.len();
    let mut res = Vec::with_capacity(n * (n - 1) / 2);

    for (i, &(ax, ay, az)) in coords.iter().enumerate() {
        for (j, &(bx, by, bz)) in coords.iter().enumerate().skip(i + 1) {
            let dx = ax.abs_diff(bx);
            let dy = ay.abs_diff(by);
            let dz = az.abs_diff(bz);
            let dist2 = dx * dx + dy * dy + dz * dz;
            res.push((dist2, i, j));
        }
    }

    res.sort_unstable_by_key(|e| e.0);
    res
}

#[inline]
fn combine_circuits(circuits: &mut Vec<Vec<usize>>, a: usize, b: usize) {
    let mut a_circuit = None;
    let mut b_circuit = None;

    for (i, circuit) in circuits.iter().enumerate() {
        if circuit.contains(&a) {
            a_circuit = Some(i);
        }
        if circuit.contains(&b) {
            b_circuit = Some(i);
        }
    }

    match (a_circuit, b_circuit) {
        (Some(a_idx), Some(b_idx)) if a_idx != b_idx => {
            let (keep, remove) = if a_idx > b_idx {
                (b_idx, a_idx)
            } else {
                (a_idx, b_idx)
            };
            let remove_circuit = circuits.remove(remove);
            circuits[keep].extend(remove_circuit);
        }
        (Some(_), Some(_)) => {}
        (Some(a_idx), None) => {
            circuits[a_idx].push(b);
        }
        (None, Some(b_idx)) => {
            circuits[b_idx].push(a);
        }
        (None, None) => {
            circuits.push(vec![a, b]);
        }
    }
}

fn part_one(input: &str, take: usize) -> u64 {
    let coords = parse_input(input);

    let connections = get_connections(&coords);

    let mut circuits: Vec<Vec<usize>> = vec![];

    for (_dist, a, b) in connections.iter().take(take).copied() {
        combine_circuits(&mut circuits, a, b);
    }

    circuits
        .iter()
        .sorted_unstable_by_key(|c| -(c.len() as isize))
        .take(3)
        .map(|c| c.len() as u64)
        .product()
}

fn part_two(input: &str) -> u64 {
    let coords = parse_input(input);
    let connections = get_connections(&coords);
    let mut circuits: Vec<Vec<usize>> = vec![];

    for (_dist, a, b) in connections {
        combine_circuits(&mut circuits, a, b);

        if circuits.len() == 1 && circuits[0].len() == coords.len() {
            return coords[a].0 * coords[b].0;
        }
    }

    0
}

fn main() {
    assert_eq!(part_one(TEST_INPUT, 10), 40);
    solved!("Day 8 part one {}", part_one(INPUT, 1000), 105952);

    assert_eq!(part_two(TEST_INPUT), 25272);
    solved!("Day 8 part two {}", part_two(INPUT), 975931446);
}
