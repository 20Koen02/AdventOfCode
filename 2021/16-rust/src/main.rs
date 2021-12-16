mod packet;
use packet::StrBits;

const INPUT: &str = include_str!("in.txt");

fn main() {
    let transmission: Vec<u8> = INPUT.to_bits();
    let (packet, _) = packet::PacketParser::parse(&transmission.as_slice());

    println!("Day 16 part one: {}", packet.sum_version_ids());
    println!("Day 16 part two: {}", packet.eval());
}
