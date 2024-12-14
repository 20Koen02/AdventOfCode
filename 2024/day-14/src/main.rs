use helper::solved;

const INPUT: &str = include_str!("in.txt");

const WIDTH: isize = 101;
const HEIGHT: isize = 103;
const HALF_WIDTH: isize = WIDTH / 2;
const HALF_HEIGHT: isize = HEIGHT / 2;

fn parse(input: &str) -> Vec<[isize; 4]> {
    input
        .lines()
        .map(|s| {
            let re = regex::Regex::new(r"-*\d+").unwrap();
            let numbers: Vec<isize> = re
                .find_iter(s)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            assert_eq!(numbers.len(), 4);
            numbers.as_slice().try_into().unwrap()
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let robots = parse(input);

    let simulated = robots
        .iter()
        .map(|[px, py, vx, vy]| {
            let x = (px + vx * 100).rem_euclid(WIDTH);
            let y = (py + vy * 100).rem_euclid(HEIGHT);
            (x, y)
        })
        .collect::<Vec<_>>();

    let mut quadrants = vec![0; 4];

    for (x, y) in simulated {
        if x == HALF_WIDTH || y == HALF_HEIGHT {
            continue;
        }
        match (x, y) {
            (..HALF_WIDTH, ..HALF_HEIGHT) => quadrants[0] += 1, // top left
            (HALF_WIDTH.., ..HALF_HEIGHT) => quadrants[1] += 1, // top right
            (..HALF_WIDTH, HALF_HEIGHT..) => quadrants[2] += 1, // bottom left
            (HALF_WIDTH.., HALF_HEIGHT..) => quadrants[3] += 1, // bottom right
        }
    }

    quadrants.iter().product()
}

fn part_two(input: &str) -> usize {
    let mut robots = parse(input);

    let mut tiles = [[0u16; HEIGHT as usize]; WIDTH as usize];

    let mut e = 0;
    loop {
        e += 1;

        robots.iter_mut().for_each(|r| {
            r[0] = (r[0] + r[2]).rem_euclid(WIDTH);
            r[1] = (r[1] + r[3]).rem_euclid(HEIGHT);

            tiles[r[0] as usize][r[1] as usize] = e;
        });

        // check how many robots are fully surrounded
        let surrounded = robots
            .iter()
            .filter(|&&[x, y, _, _]| {
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .all(|(x, y)| {
                        tiles.get(*x as usize).and_then(|row| row.get(*y as usize)) == Some(&e)
                    })
            })
            .count();

        // if there are more than 100 robots fully surrounded, it's probably the easter egg
        if surrounded > 100 {
            return e as usize;
        }
    }
}

fn main() {
    solved!("Day 14 part one: {}", part_one(INPUT), 230436441);
    solved!("Day 14 part two: {}", part_two(INPUT), 8270);
}
