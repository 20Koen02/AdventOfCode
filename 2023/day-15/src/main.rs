use helper::solved;

const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
const INPUT: &str = include_str!("in.txt");

fn get_hash_value(str: &str) -> usize {
    str.chars().fold(0, |mut acc, code| {
        acc += code as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn part_one(input: &str) -> usize {
    input.split(',').map(get_hash_value).sum()
}

fn part_two(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];

    for s in input.split(',') {
        if let Some((label, focal_str)) = s.split_once('=') {
            let focal: u32 = focal_str.parse().unwrap();
            let hash = get_hash_value(label);

            if let Some(pos) = boxes[hash].iter().position(|entry| entry.0 == label) {
                let elem = &mut boxes[hash][pos];
                *elem = (label, focal);
            } else {
                boxes[hash].push((label, focal));
            }
        } else if s.ends_with('-') {
            let label = s.strip_suffix('-').unwrap();
            let hash = get_hash_value(label);

            if let Some(pos) = boxes[hash].iter().position(|entry| entry.0 == label) {
                boxes[hash].remove(pos);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .fold(0, |mut acc, (box_num, box_slots)| {
            for (slot_num, slot) in box_slots.iter().enumerate() {
                acc += (box_num + 1) * (slot_num + 1) * slot.1 as usize;
            }
            acc
        })
}

fn main() {
    assert_eq!(part_one(TEST_INPUT), 1320);
    solved!("Day 15 part 1: {}", part_one(INPUT), 513172);

    assert_eq!(part_two(TEST_INPUT), 145);
    solved!("Day 15 part 2: {}", part_two(INPUT), 237806);
}
