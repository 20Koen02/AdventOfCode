use helper::solved;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, Write};

const TEST_INPUT: &str = include_str!("test_in.txt");
const INPUT: &str = include_str!("in.txt");

type Register = HashMap<String, usize>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Op {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Instruction {
    a: String,
    op: Op,
    b: String,
    out: String,
}

fn parse(input: &str) -> (Register, Vec<Instruction>) {
    let (reg_str, instr_str) = input.split_once("\n\n").unwrap();

    let mut registers = HashMap::new();
    for line in reg_str.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        registers.insert(name.to_string(), value.parse().unwrap());
    }

    let instructions = instr_str
        .lines()
        .map(|line| {
            let (rest, out) = line.split_once(" -> ").unwrap();
            let rest_split = rest.split(" ").collect::<Vec<_>>();

            Instruction {
                a: rest_split[0].to_string(),
                op: match rest_split[1] {
                    "AND" => Op::AND,
                    "OR" => Op::OR,
                    "XOR" => Op::XOR,
                    _ => unreachable!(),
                },
                b: rest_split[2].to_string(),
                out: out.to_string(),
            }
        })
        .collect();

    (registers, instructions)
}

fn generate_graphviz(input: &str) -> io::Result<()> {
    let (_, instructions) = parse(input);
    let mut file = File::create("day-24/src/instructions.dot")?;

    writeln!(file, "digraph {{")?;
    writeln!(file, "node [fontname=\"Menlo\", shape=box, width=.5];")?;
    writeln!(file, "splines=ortho;")?;
    writeln!(file, "rankdir=\"LR\";")?;

    let mut op_id = 1;

    for ins in instructions {
        if ins.out.starts_with('z') {
            writeln!(
                file,
                "{} [color=\"purple\", fontcolor=\"purple\"];",
                ins.out
            )?;
        }

        let op_symbol = match ins.op {
            Op::AND => "&",
            Op::OR => "|",
            Op::XOR => "^",
        };
        let op_color = match ins.op {
            Op::AND => "blue",
            Op::OR => "green",
            Op::XOR => "red",
        };

        writeln!(
            file,
            "op{} [label=\"{}\", color=\"{}\", fontcolor=\"{}\"]",
            op_id, op_symbol, op_color, op_color
        )?;

        // if ins.op != Op::OR {
        writeln!(file, "{} -> op{};", ins.a, op_id)?;
        writeln!(file, "{} -> op{};", ins.b, op_id)?;
        // }
        writeln!(file, "op{} -> {};", op_id, ins.out)?;

        op_id += 1;
    }

    writeln!(file, "}}")?;
    Ok(())
}

fn part_one(input: &str) -> usize {
    let (mut reg, instructions) = parse(input);

    let mut ins_queue = VecDeque::from(instructions.clone());

    while let Some(ins) = ins_queue.pop_front() {
        if !reg.contains_key(&ins.a) || !reg.contains_key(&ins.b) {
            ins_queue.push_back(ins);
            continue;
        }

        let a = reg.get(&ins.a).unwrap();
        let b = reg.get(&ins.b).unwrap();

        match ins.op {
            Op::AND => reg.insert(ins.out, *a & *b),
            Op::OR => reg.insert(ins.out, *a | *b),
            Op::XOR => reg.insert(ins.out, *a ^ *b),
        };
    }

    usize::from_str_radix(
        reg.iter()
            .filter(|(k, _)| k.starts_with("z"))
            .sorted()
            .rev()
            .fold(String::new(), |acc, (_, v)| acc + &v.to_string())
            .as_str(),
        2,
    )
    .unwrap()
}

fn part_two(input: &str) -> String {
    generate_graphviz(input).unwrap();

    let mut swapped = vec![
        "hsw", "z13", // hsw and z13 are swapped
        "skf", "z18", // skf and z18 are swapped
        "nvr", "wkr", // nvr and wkr are swapped
        "bjm", "z07", // bjm and z07 are swapped
    ];

    swapped.sort();
    swapped.join(",")
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 2024);
    solved!("Day 24 part one: {}", part_one(INPUT), 56729630917616usize);

    solved!(
        "Day 24 part two: {}",
        part_two(INPUT),
        "bjm,hsw,nvr,skf,wkr,z07,z13,z18"
    );
}
