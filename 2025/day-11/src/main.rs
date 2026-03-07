use helper::solved;
use std::collections::HashMap;

mod viz;

const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
const TEST_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
const INPUT: &str = include_str!("in.txt");

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let children: Vec<&str> = rest.split_whitespace().collect();
        graph.insert(name, children);
    }

    graph
}

fn part_one(input: &str) -> u64 {
    let graph = parse_graph(input);

    fn count_paths<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        memo: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if let Some(&res) = memo.get(node) {
            return res;
        }

        let res = match graph.get(node) {
            None => 1,
            Some(children) => children
                .iter()
                .map(|&child| count_paths(child, graph, memo))
                .sum(),
        };

        memo.insert(node, res);
        res
    }

    count_paths("you", &graph, &mut HashMap::new())
}

fn part_two(input: &str) -> u64 {
    let graph = parse_graph(input);

    fn dfs<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        visited_dac: bool,
        visited_fft: bool,
        memo: &mut HashMap<(&'a str, bool, bool), u64>,
    ) -> u64 {
        let visited_dac = visited_dac || node == "dac";
        let visited_fft = visited_fft || node == "fft";

        if let Some(&res) = memo.get(&(node, visited_dac, visited_fft)) {
            return res;
        }

        let res = match graph.get(node) {
            None => (visited_dac && visited_fft) as u64,
            Some(children) => children
                .iter()
                .map(|&child| dfs(child, graph, visited_dac, visited_fft, memo))
                .sum(),
        };

        memo.insert((node, visited_dac, visited_fft), res);
        res
    }

    dfs("svr", &graph, false, false, &mut HashMap::new())
}

fn main() {
    viz::generate_graphviz(&parse_graph(INPUT)).unwrap();

    assert_eq!(part_one(TEST_INPUT), 5);
    solved!("Day 11 part one {}", part_one(INPUT), 615);

    assert_eq!(part_two(TEST_INPUT_2), 2);
    solved!("Day 11 part two {}", part_two(INPUT), 303012373210128u64);
}
