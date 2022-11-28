use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("in.txt");

type Beacon = [i16; 3];

fn get_input() -> Vec<Vec<Beacon>> {
    INPUT
        .split("\n\n")
        .map(|f| {
            f.split("\n")
                .skip(1)
                .map(|l| {
                    let mut iter = l.split(",");
                    [
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().parse().unwrap(),
                        iter.next().unwrap().parse().unwrap(),
                    ]
                })
                .collect()
        })
        .collect()
}

fn distance(p1: &Beacon, p2: &Beacon) -> u16 {
    let dx = (p1[0] - p2[0]) as i32;
    let dy = (p1[1] - p2[1]) as i32;
    let dz = (p1[2] - p2[2]) as i32;
    ((dx * dx + dy * dy + dz * dz) as f32).sqrt() as u16
}

fn distance_taxi(p1: &Beacon, p2: &Beacon) -> u16 {
    let dx = (p1[0] - p2[0]) as i32;
    let dy = (p1[1] - p2[1]) as i32;
    let dz = (p1[2] - p2[2]) as i32;
    (dx.abs() + dy.abs() + dz.abs()) as u16
}

fn get_config(scanner: &HashSet<Beacon>) -> HashMap<Beacon, HashSet<u16>> {
    let mut config: HashMap<Beacon, HashSet<u16>> = HashMap::new();

    for p1 in scanner {
        for p2 in scanner {
            if p1 == p2 {
                continue;
            }
            (config.entry(p1.clone()).or_insert(HashSet::new())).insert(distance(&p1, &p2));
        }
    }
    config
}

fn get_common_pt_num(
    s1: &HashMap<Beacon, HashSet<u16>>,
    s2: &HashMap<Beacon, HashSet<u16>>,
) -> u16 {
    s1.keys()
        .map(|p1| {
            s2.keys()
                .map(|p2| {
                    s1.get(p1)
                        .unwrap()
                        .intersection(s2.get(p2).unwrap())
                        .count() as u16
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn align(
    config1: &HashMap<Beacon, HashSet<u16>>,
    config2: &HashMap<Beacon, HashSet<u16>>,
) -> ([i16; 3], HashMap<u8, (u8, i8)>) {
    let mut mapping: HashMap<Beacon, Beacon> = HashMap::new();
    for p1 in config1.keys() {
        for p2 in config2.keys() {
            if config1
                .get(p1)
                .unwrap()
                .intersection(config2.get(p2).unwrap())
                .count()
                > 10
            {
                mapping.insert(*p1, *p2);
            }
        }
    }

    let p1 = mapping.keys().last().unwrap();
    let p2 = mapping.get(p1).unwrap();

    let p1_mod: Vec<i16> = (0..3)
        .map(|i| {
            (p1[i] as f32
                - mapping
                    .keys()
                    .map(|x| x[i] as i32)
                    .collect::<Vec<_>>()
                    .iter()
                    .sum::<i32>() as f32
                    / mapping.keys().count() as f32)
                .round() as i16
        })
        .collect();

    let p2_mod: Vec<i16> = (0..3)
        .map(|i| {
            (p2[i] as f32
                - mapping
                    .values()
                    .map(|x| x[i] as i32)
                    .collect::<Vec<_>>()
                    .iter()
                    .sum::<i32>() as f32
                    / mapping.keys().count() as f32)
                .round() as i16
        })
        .collect();

    let mut rot: HashMap<u8, (u8, i8)> = HashMap::new();
    for i in 0..3 {
        for j in 0..3 {
            if p1_mod[i].abs() == p2_mod[j].abs() {
                rot.insert(i as u8, (j as u8, (p1_mod[i] / p2_mod[j]) as i8));
            }
        }
    }

    let mut p2_rot: [i16; 3] = [0; 3];

    for i in 0..3 {
        p2_rot[i] =
            p2[rot.get(&(i as u8)).unwrap().0 as usize] * rot.get(&(i as u8)).unwrap().1 as i16;
    }

    let mut trans: [i16; 3] = [0; 3];
    for i in 0..3 {
        trans[i] = p2_rot[i] - p1[i];
    }

    (trans, rot)
}

fn transform_points(
    trans: &[i16; 3],
    rot: &HashMap<u8, (u8, i8)>,
    points: &Vec<Beacon>,
) -> HashSet<Beacon> {
    points
        .into_iter()
        .map(|p| {
            let mut t = [0; 3];
            for i in 0..3 {
                t[i] = p[rot[&(i as u8)].0 as usize] * rot[&(i as u8)].1 as i16 - trans[i];
            }
            t
        })
        .collect()
}

fn part1() -> Vec<[i16; 3]> {
    let mut scanners = get_input();
    let mut grid: HashSet<Beacon> = HashSet::from_iter(scanners[0].iter().cloned());
    scanners.remove(0);
    let mut pos: Vec<[i16; 3]> = Vec::new();

    while scanners.len() > 0 {
        let grid_config = get_config(&grid);

        let scanners_common: Vec<_> = scanners
            .iter()
            .map(|s| {
                get_common_pt_num(
                    &grid_config,
                    &get_config(&HashSet::from_iter(s.iter().cloned())),
                )
            })
            .collect();

        let s = scanners_common
            .iter()
            .position(|x| x == scanners_common.iter().max().unwrap())
            .unwrap();

        let (trans, rot) = align(
            &grid_config,
            &get_config(&HashSet::from_iter(scanners[s].iter().cloned())),
        );

        grid.extend(&transform_points(&trans, &rot, &scanners[s]));

        scanners.remove(s);
        pos.push(trans);
    }

    assert_eq!(grid.len(), 365);
    println!("Day 19 part 1: {}", grid.len());
    pos
}

fn part_2(pos: &Vec<[i16; 3]>) {
    let mut dists: Vec<u16> = Vec::new();
    for p1 in pos {
        for p2 in pos {
            dists.push(distance_taxi(p1, p2));
        }
    }
    assert_eq!(*dists.iter().max().unwrap(), 11060 as u16);
    println!("Day 19 part 2: {}", dists.iter().max().unwrap());
}

fn main() {
    let scanner_pos = part1();
    part_2(&scanner_pos);
}

