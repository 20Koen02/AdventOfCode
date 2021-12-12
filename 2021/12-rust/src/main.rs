use std::collections::HashMap;

const INPUT: &str = include_str!("in.txt");

fn get_edges() -> HashMap<&'static str, Vec<&'static str>> {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    INPUT.trim().split("\n").for_each(|line| {
        let parts = line.split("-").collect::<Vec<&str>>();
        let (a, b) = (parts[0], parts[1]);
        edges.entry(a).or_insert(vec![]).push(b);
        edges.entry(b).or_insert(vec![]).push(a);
    });
    return edges;
}

fn is_small_cave(cave: &str) -> bool {
    return cave != cave.to_uppercase();
}

fn is_visited(cave: &str, path: &Vec<&str>) -> bool {
    return path.contains(&cave);
}

fn all_paths_count(
    edges: &HashMap<&str, Vec<&str>>,
    small_cave: bool,
    cur_node: &str,
    path: &Vec<&str>,
) -> usize {
    let mut small_cave = small_cave;
    if cur_node == "end" {
        return 1;
    };

    if is_small_cave(cur_node) && is_visited(cur_node, &path) {
        small_cave = false;
    }

    return edges
        .get(cur_node)
        .unwrap()
        .iter()
        .filter(|&neighbor| {
            !is_small_cave(neighbor)
                || !is_visited(neighbor, &path)
                || (small_cave && neighbor != &"start")
        })
        .map(|neighbor| {
            let mut path = path.clone();
            path.push(cur_node);
            return all_paths_count(edges, small_cave, neighbor, &path);
        })
        .sum();
}

fn main() {
    let edges: HashMap<&str, Vec<&str>> = get_edges();
    println!(
        "Day 12 part one: {}",
        all_paths_count(&edges, false, "start", &vec![])
    ); // 3450
    println!(
        "Day 12 part two: {}",
        all_paths_count(&edges, true, "start", &vec![])
    ); // 96528
}
