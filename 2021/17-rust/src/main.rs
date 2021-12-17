const X1: i32 = 155;
const X2: i32 = 182;
const Y1: i32 = -117;
const Y2: i32 = -67;

fn simulate(mut vx: i32, mut vy: i32) -> i32 {
    let mut x = 0;
    let mut y = 0;
    while y > -Y1.abs() {
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
        if X1 <= x && x <= X2 && -Y1.abs() <= y && y <= -Y2.abs() {
            return 1;
        }
    }
    return 0;
}

fn part_two() -> i32 {
    (-Y1.abs()..Y1.abs()).fold(0, |acc, vy| {
        acc + (1..(X2 + 1)).fold(0, |acc, vx| acc + simulate(vx, vy))
    })
}

fn main() {
    println!("Day 16 part one: {}", Y1.abs() * (Y1.abs() - 1) / 2);
    println!("Day 16 part two: {}", part_two());
}
