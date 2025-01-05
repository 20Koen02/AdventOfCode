use helper::solved;
use itertools::{sorted, Itertools};
use std::collections::{HashMap, HashSet};

const TEST_INPUT: &str = include_str!("test_in.txt");
const INPUT: &str = include_str!("in.txt");

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        map.entry(a.to_string())
            .or_insert_with(HashSet::new)
            .insert(b.to_string());
        map.entry(b.to_string())
            .or_insert_with(HashSet::new)
            .insert(a.to_string());
    }

    map
}

fn bron_kerbosch_pivoting(
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    graph: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    let pivot = p.union(&x).next().unwrap().clone();
    let pivot_neighbors = graph.get(&pivot).cloned().unwrap_or_default();

    for vertex in &p - &pivot_neighbors {
        let mut r_new = r.clone();
        r_new.insert(vertex.clone());

        let neighbors = graph.get(&vertex).cloned().unwrap_or_default();

        let p_new = &p & &neighbors;
        let x_new = &x & &neighbors;

        bron_kerbosch_pivoting(r_new, p_new, x_new, graph, cliques);

        p.remove(&vertex);
        x.insert(vertex);
    }
}

fn part_one(input: &str) -> usize {
    let map = parse(input);

    let mut inter_connected = HashSet::new();

    for (node, edges) in map.iter().filter(|(k, _)| k.starts_with('t')) {
        edges.iter().cartesian_product(edges).for_each(|(a, b)| {
            if map.get(a).unwrap().contains(b) {
                inter_connected.insert(sorted([node, a, b]).collect_vec());
            }
        });
    }

    inter_connected.len()
}

fn part_two(input: &str) -> String {
    let map = parse(input);

    let mut cliques = Vec::new();

    bron_kerbosch_pivoting(
        HashSet::new(),
        map.keys().cloned().collect(),
        HashSet::new(),
        &map,
        &mut cliques,
    );

    cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 7);
    solved!("Day 23 part one: {}", part_one(INPUT), 1308);

    assert_eq!(part_two(TEST_INPUT), "co,de,ka,ta");
    solved!(
        "Day 23 part two: {}",
        part_two(INPUT),
        "bu,fq,fz,pn,rr,st,sv,tr,un,uy,zf,zi,zy"
    );
}
