use helper::solved;

const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

const INPUT: &str = include_str!("in.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
}

fn parse(input: &str, part_two: bool) -> (Vec<Vec<Node>>, Vec<(isize, isize)>, (isize, isize)) {
    let (map_str, moves_str) = input.trim().split_once("\n\n").unwrap();

    let mut robot: (isize, isize) = (0, 0);
    let mut map = vec![vec![]; map_str.lines().count()];
    for (y, line) in map_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                robot = (x as isize * (if part_two { 2 } else { 1 }), y as isize);
            }
            if part_two {
                map[y].extend(match c {
                    '#' => [Node::Wall, Node::Wall],
                    'O' => [Node::BoxLeft, Node::BoxRight],
                    '.' | '@' => [Node::Empty, Node::Empty],
                    _ => unreachable!(),
                });
            } else {
                map[y].push(match c {
                    '#' => Node::Wall,
                    'O' => Node::Box,
                    '.' | '@' => Node::Empty,
                    _ => unreachable!(),
                });
            }
        }
    }

    let moves = moves_str
        .chars()
        .filter_map(|c| match c {
            '^' => Some((0, -1)),
            'v' => Some((0, 1)),
            '>' => Some((1, 0)),
            '<' => Some((-1, 0)),
            _ => None,
        })
        .collect();

    (map, moves, robot)
}

fn move_check(map: &mut Vec<Vec<Node>>, robot: &mut (isize, isize), mv: (isize, isize)) -> bool {
    let (dx, dy) = mv;

    let mut to_check = (robot.0 + dx, robot.1 + dy);
    if let Some(next_node) = map
        .get(to_check.1 as usize)
        .and_then(|row| row.get(to_check.0 as usize))
    {
        let vertical = mv.0 == 0;
        let res = match next_node {
            Node::Wall => false,
            Node::BoxLeft | Node::BoxRight if vertical => {
                let other = if next_node == &Node::BoxLeft { 1 } else { -1 };

                // first move_check is a fake move, so we can check if we can move to the other box
                move_check(&mut map.clone(), &mut to_check.clone(), mv)
                    && move_check(map, &mut (to_check.0 + other, to_check.1), mv)
                    && move_check(map, &mut to_check, mv)
            }
            Node::Box | Node::BoxLeft | Node::BoxRight => move_check(map, &mut to_check, mv),
            Node::Empty => true,
        };
        if res {
            map[to_check.1 as usize][to_check.0 as usize] = map[robot.1 as usize][robot.0 as usize];
            map[robot.1 as usize][robot.0 as usize] = Node::Empty;
        }
        res
    } else {
        false
    }
}

fn gps_result(map: &Vec<Vec<Node>>) -> usize {
    let mut res = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Node::Box || map[y][x] == Node::BoxLeft {
                res += 100 * y + x;
            }
        }
    }

    res
}

fn solve(input: &str, part_two: bool) -> usize {
    let (mut map, moves, mut robot) = parse(input, part_two);

    for mv in moves {
        if move_check(&mut map, &mut robot, mv) {
            robot.0 += mv.0;
            robot.1 += mv.1;
        }
    }

    gps_result(&map)
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 10092);
    solved!("Day 15 part one: {}", solve(INPUT, false), 1463512);

    assert_eq!(solve(TEST_INPUT, true), 9021);
    solved!("Day 15 part two: {}", solve(INPUT, true), 1486520);
}
