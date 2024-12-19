use helper::solved;

const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

const TEST_INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

const INPUT: &str = include_str!("in.txt");

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    pointer: usize,
    output: Vec<usize>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let mut lines = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.split_once(": ").unwrap().1);

        Self {
            a: lines.next().unwrap().parse().unwrap(),
            b: lines.next().unwrap().parse().unwrap(),
            c: lines.next().unwrap().parse().unwrap(),
            program: lines
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
            pointer: 0,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.pointer < self.program.len() {
            let instruction = self.program[self.pointer];

            match instruction {
                0 => self.a >>= self.get_combo(),
                1 => self.b ^= self.get_operand(),
                2 => self.b = self.get_combo() % 8,
                3 if self.a != 0 => {
                    self.pointer = self.get_operand();
                    continue;
                }
                4 => self.b ^= self.c,
                5 => self.output.push(self.get_combo() % 8),
                6 => self.b = self.a >> self.get_combo(),
                7 => self.c = self.a >> self.get_combo(),
                _ => {}
            };

            self.pointer += 2;
        }
    }

    fn get_operand(&self) -> usize {
        self.program[self.pointer + 1]
    }

    fn get_combo(&self) -> usize {
        let operand = self.get_operand();
        match operand {
            ..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn get_output_string(&self) -> String {
        self.output
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn is_copy(&self) -> bool {
        self.program == self.output
    }

    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.pointer = 0;
        self.output.clear();
    }
}

fn part_one(input: &str) -> String {
    let mut comp = Computer::parse(input);
    comp.run();
    comp.get_output_string()
}

fn part_two(input: &str) -> usize {
    let mut comp = Computer::parse(input);

    let mut a: usize = 0;
    loop {
        comp.reset();
        comp.a = a;
        comp.run();

        if comp.is_copy() {
            return a;
        }

        for (i, op) in comp.program.iter().enumerate().rev() {
            if comp.output.len() < i || comp.output[i] != *op {
                a += 8usize.pow(i as u32);
                break;
            }
        }
    }
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
    solved!("Day 17 part one: {}", part_one(INPUT), "2,1,0,4,6,2,4,2,0");

    assert_eq!(part_two(TEST_INPUT2), 117440);
    solved!(
        "Day 17 part two: {}",
        part_two(INPUT),
        109685330781408_usize
    );
}
