use helper::solved;

const INPUT: &str = include_str!("in.txt");

type Instruction = (String, isize);

fn parse_instructions() -> Vec<Instruction> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap_or("0").parse().unwrap(),
            )
        })
        .collect()
}

fn solve(instructions: Vec<Instruction>) -> (isize, String) {
    let mut x = 1;
    let mut clock = 0;
    let mut signal = 0;
    let mut crt = String::new();

    let mut tick = |x: &isize| {
        if clock % 40 == 0 {
            crt.push('\n');
        }
        if (clock % 40_isize).abs_diff(*x) <= 1 {
            crt.push('#');
        } else {
            crt.push('.');
        }

        clock += 1;

        if (clock - 20) % 40 == 0 {
            signal += clock * x;
        }
    };

    for (instruction, val) in instructions {
        tick(&x);
        if instruction == "addx" {
            tick(&x);
            x += val;
        }
    }

    (signal, crt)
}

fn main() {
    let instructions = parse_instructions();
    let (part1, part2) = solve(instructions);
    solved!("Day 10 part 1: {}", part1, 13760);
    solved!("Day 10 part 2: {}", part2, "\n###..####.#..#.####..##..###..####.####.\n#..#.#....#.#.....#.#..#.#..#.#....#....\n#..#.###..##.....#..#....#..#.###..###..\n###..#....#.#...#...#....###..#....#....\n#.#..#....#.#..#....#..#.#....#....#....\n#..#.#....#..#.####..##..#....####.#....");
}
