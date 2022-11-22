use itertools::Itertools;

type SfNum = Vec<(u16, u16)>;

const INPUT: &str = include_str!("in.txt");

fn get_input() -> Vec<SfNum> {
    INPUT
        .lines()
        .map(|line| {
            let mut depth = 0;
            line.chars().fold(vec![], |mut acc: SfNum, c| {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    num if num.is_digit(10) => {
                        acc.push((num.to_digit(10).unwrap().try_into().unwrap(), depth))
                    }
                    _ => (),
                }
                acc
            })
        })
        .collect()
}

fn magnitude(mut num: SfNum) -> u16 {
    (1..=4)
        .rev()
        .for_each(|d| while magnitude_loop(&mut num, d) {});
    num[0].0
}

fn magnitude_loop(num: &mut SfNum, d: u16) -> bool {
    for (i, (&(aval, adepth), &(bval, bdepth))) in num.iter().tuple_windows().enumerate() {
        if adepth == d && bdepth == d {
            num[i] = ((3 * aval + 2 * bval), adepth - 1);
            num.remove(i + 1);
            return true;
        }
    }
    false
}

fn add_reduce_list(nums: &[SfNum]) -> SfNum {
    let mut cur = nums[0].clone();
    nums.iter()
        .skip(1)
        .for_each(|num| add_reduce(&mut cur, num));
    cur
}

fn add_reduce(num: &mut SfNum, toadd: &[(u16, u16)]) {
    num.extend(toadd);
    num.iter_mut().for_each(|(_, depth)| *depth += 1);
    while explode(num) || split(num) {}
}

fn explode(num: &mut SfNum) -> bool {
    for (i, (&(aval, adepth), &(bval, bdepth))) in num.iter().tuple_windows().enumerate() {
        if adepth == 5 && bdepth == 5 {
            if num.get(i.saturating_sub(1)).is_some() && i.saturating_sub(1) != i {
                num.get_mut(i - 1).unwrap().0 += aval;
            }
            if num.get(i + 2).is_some() {
                num.get_mut(i + 2).unwrap().0 += bval;
            }
            num.drain(i..i + 2);
            num.insert(i, (0, 4));
            return true;
        }
    }
    false
}

fn split(num: &mut SfNum) -> bool {
    for (i, &(val, depth)) in num.iter().enumerate() {
        if val > 9 {
            num.remove(i);
            num.insert(i, ((((val as f64) / 2f64).floor()) as u16, depth + 1));
            num.insert(i + 1, ((((val as f64) / 2f64).ceil()) as u16, depth + 1));
            return true;
        }
    }
    false
}

fn solve(x: &Vec<SfNum>) -> u16 {
    magnitude(add_reduce_list(&x))
}

fn main() {
    let lines: Vec<SfNum> = get_input();

    println!("Part 1: {}", solve(&lines));

    println!(
        "Part 1: {}",
        lines
            .into_iter()
            .permutations(2)
            .map(|perm| solve(&perm))
            .max()
            .unwrap()
    );
}
