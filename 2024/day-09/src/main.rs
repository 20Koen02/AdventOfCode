use helper::solved;

const TEST_INPUT: &str = "2333133121414131402";
const INPUT: &str = include_str!("in.txt");

type FS = Vec<(usize, Option<usize>)>;

fn checksum(mut files: FS) -> usize {
    let mut i = files.len() - 1;
    while i > 0 {
        let (size, id) = files[i];

        if id.is_none() {
            i -= 1;
            continue;
        }
        if let Some(j) = files[0..i]
            .iter()
            .position(|&(s, id)| id.is_none() && size <= s)
        {
            let s = files[j].0;
            files[j] = (size, id);
            files[i] = (size, None);
            if size < s {
                files.insert(j + 1, (s - size, None));
            }
        }
        i -= 1;
    }
    files
        .iter()
        .flat_map(|&(s, id)| (0..s).map(move |_| id))
        .enumerate()
        .map(|(i, id)| match id {
            Some(id) => i * id as usize,
            None => 0,
        })
        .sum()
}

fn solve(input: &str, part_two: bool) -> usize {
    let mut files: FS = Vec::new();

    let mut file_id: i32 = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let id: Option<usize> = if i % 2 == 0 {
            file_id += 1;
            Some((file_id - 1) as usize)
        } else {
            None
        };

        if part_two {
            files.push((c.to_digit(10).unwrap() as usize, id));
        } else {
            files.extend((0..c.to_digit(10).unwrap()).map(|_| (1, id)));
        }
    }

    checksum(files)
}

fn main() {
    assert_eq!(solve(TEST_INPUT, false), 1928);
    solved!(
        "Day 9 part one: {}",
        solve(INPUT, false),
        6370402949053usize
    );

    assert_eq!(solve(TEST_INPUT, true), 2858);
    solved!("Day 9 part two: {}", solve(INPUT, true), 6398096697992usize);
}
