fn main() {
    let input = "1721\n979\n366\n299\n675\n1456";
    
    let nums = parse_input(input);
    
    println!("Part 1: {}", solve_part1(&nums));
    println!("Part 2: {}", solve_part2(&nums));
}

fn parse_input(input: &str) -> Vec<i32> {
    let input_lines: Vec<&str> = input.split("\n").collect();
    
    let nums: Vec<i32> = input_lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    
    nums
}

fn solve_part1(nums: &Vec<i32>) -> i32 {
    for num1 in nums.iter() {
        for num2 in nums.iter() {
            if num1 + num2 == 2020 {
                return num1 * num2
            }
        }
    }
    panic!()
}

fn solve_part2(nums: &Vec<i32>) -> i32 {
    for num1 in nums.iter() {
        for num2 in nums.iter() {
            for num3 in nums.iter() {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3
                }
            }
        }
    }
    panic!()
}
