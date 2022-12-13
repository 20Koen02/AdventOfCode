use helper::solved;
use itertools::Itertools;
use serde_json::Value as Packet;
use std::cmp::Ordering;

const INPUT: &str = include_str!("in.txt");

fn cmp_vec(l: &[Packet], r: &[Packet]) -> Ordering {
    for i in 0..l.len().max(r.len()) {
        match (l.get(i), r.get(i)) {
            (Some(l), Some(r)) => match cmp(l, r) {
                Ordering::Equal => (),
                o => return o,
            },
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => unreachable!(),
        }
    }
    Ordering::Equal
}

fn cmp(l: &Packet, r: &Packet) -> Ordering {
    match (l, r) {
        (Packet::Number(l), Packet::Number(r)) => l.as_u64().unwrap().cmp(&r.as_u64().unwrap()),
        (Packet::Number(l), r) => cmp(&Packet::Array(vec![Packet::Number(l.clone())]), r),
        (l, Packet::Number(r)) => cmp(l, &Packet::Array(vec![Packet::Number(r.clone())])),
        (Packet::Array(l), Packet::Array(r)) => cmp_vec(l, r),
        _ => unreachable!(),
    }
}

fn part1(packets: &[Packet]) -> usize {
    packets
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (l, r))| cmp(l, r).is_lt())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(packets: &[Packet]) -> usize {
    let mut packets = packets.to_vec();

    let divider_packets = vec![
        serde_json::from_str::<Packet>("[[2]]").unwrap(),
        serde_json::from_str::<Packet>("[[6]]").unwrap(),
    ];
    packets.extend(divider_packets.iter().cloned());

    packets.sort_by(cmp);

    packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.contains(packet))
        .map(|(i, _)| i + 1)
        .product()
}

fn main() {
    let packets = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Packet>(line).unwrap())
        .collect::<Vec<Packet>>();

    solved!("Day 13 part 1: {}", part1(&packets), 6415);
    solved!("Day 13 part 2: {}", part2(&packets), 20056);
}
