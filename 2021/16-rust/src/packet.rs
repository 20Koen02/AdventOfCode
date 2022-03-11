#[derive(Debug)]
pub enum Packet {
  Literal {
    version: u8,
    value: usize,
  },
  Operator {
    version: u8,
    type_id: u8,
    packets: Vec<Packet>,
  },
}

impl Packet {
  pub fn sum_version_ids(&self) -> usize {
    match self {
      Packet::Literal { version, .. } => *version as usize,
      Packet::Operator {
        version, packets, ..
      } => packets
        .iter()
        .fold(*version as usize, |acc, sub| acc + sub.sum_version_ids()),
    }
  }

  pub fn eval(&self) -> usize {
    match self {
      Packet::Literal { value, .. } => *value,
      Packet::Operator {
        type_id, packets, ..
      } => match type_id {
        0 => packets.iter().fold(0, |acc, sub| acc + sub.eval()),
        1 => packets.iter().fold(1, |acc, sub| acc * sub.eval()),
        2 => packets.iter().map(|sub| sub.eval()).min().unwrap(),
        3 => packets.iter().map(|sub| sub.eval()).max().unwrap(),
        5 => (packets[0].eval() > packets[1].eval()) as usize,
        6 => (packets[0].eval() < packets[1].eval()) as usize,
        7 => (packets[0].eval() == packets[1].eval()) as usize,
        _ => unreachable!(),
      },
    }
  }
}

type ParseResult = (Packet, usize);

pub trait PacketParser {
  fn parse(&self) -> ParseResult;
  fn parse_literal(&self, version: u8) -> ParseResult;
  fn parse_subpackets(&self, version: u8, type_id: u8) -> ParseResult;
  fn to_int(&self) -> usize;
}

impl PacketParser for &[u8] {
  fn parse(&self) -> ParseResult {
    let version = (&self[0..3]).to_int() as u8;
    let type_id = (&self[3..6]).to_int() as u8;
    let remaining = &self[6..];
    match type_id {
      4 => remaining.parse_literal(version),
      _ => remaining.parse_subpackets(version, type_id),
    }
  }

  fn parse_literal(&self, version: u8) -> ParseResult {
    let mut bits: Vec<u8> = vec![];
    let mut end = 0;
    for chunk in self.chunks(5) {
      end += 5;
      bits.extend(&chunk[1..5]);
      if chunk[0] == 0 {
        return (
          Packet::Literal {
            version,
            value: bits.as_slice().to_int(),
          },
          end + 6,
        );
      }
    }
    unreachable!();
  }

  fn parse_subpackets(&self, version: u8, type_id: u8) -> ParseResult {
    let length_type_id = self[0];
    let mut start = 0;

    let (packets, end) = match length_type_id {
      0 => {
        let len_of_bits = (&self[1..16]).to_int();
        let sub_bits = &self[16..(16 + len_of_bits)];
        let mut subs = Vec::new();
        while start < len_of_bits {
          let (sub, end) = (&sub_bits[start..]).parse();
          start += end;
          subs.push(sub);
        }
        (subs, start + 16)
      }
      1 => {
        let num_of_packets = (&self[1..12]).to_int();
        let sub_bits = &self[12..];
        let subs = (0..num_of_packets)
          .map(|_| {
            let (sub, end) = (&sub_bits[start..]).parse();
            start += end;
            sub
          })
          .collect();
        (subs, start + 12)
      }
      _ => unreachable!(),
    };

    (
      Packet::Operator {
        version,
        type_id,
        packets,
      },
      end + 6,
    )
  }

  fn to_int(&self) -> usize {
    self
      .iter()
      .rev()
      .enumerate()
      .fold(0, |acc, (n, &bit)| acc + ((bit as usize) << n))
  }
}

pub trait StrBits {
  fn to_bits(&self) -> Vec<u8>;
}

impl StrBits for &'static str {
  fn to_bits(&self) -> Vec<u8> {
    (*self).trim().chars().fold(vec![], |mut acc, hex| {
      let dec = hex.to_digit(16).unwrap() as u8;
      acc.extend(vec![dec >> 3, (dec >> 2) & 1, (dec >> 1) & 1, dec & 1]);
      acc
    })
  }
}
