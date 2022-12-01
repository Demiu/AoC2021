use anyhow::{Context, Result};

use crate::parse::ascii_digit_to_value;

type SolverInput = Packet;

enum ComplexKind {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

enum PacketContent {
    Literal(u64),
    Complex(ComplexKind, Vec<Packet>),
}

pub struct Packet {
    version: u8,
    content: PacketContent,
    length: usize,
}

impl From<u8> for ComplexKind {
    fn from(v: u8) -> Self {
        use ComplexKind::*;
        match v {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => Greater,
            6 => Less,
            7 => Equal,
            _ => panic!("Cannot convert value of {} into a PacketKind", v),
        }
    }
}

impl Packet {
    fn version_sum(&self) -> u32 {
        if let PacketContent::Complex(_, inner_packets) = &self.content {
            self.version as u32 + inner_packets.iter().map(Packet::version_sum).sum::<u32>()
        } else {
            self.version as u32
        }
    }

    fn evaluate(&self) -> u64 {
        use ComplexKind::*;
        match &self.content {
            PacketContent::Literal(val) => *val,
            PacketContent::Complex(kind, subpackets) => match kind {
                Sum => subpackets.iter().map(Packet::evaluate).sum(),
                Product => subpackets.iter().map(Packet::evaluate).product(),
                Minimum => subpackets.iter().map(Packet::evaluate).min().unwrap(),
                Maximum => subpackets.iter().map(Packet::evaluate).max().unwrap(),
                Greater => (subpackets[0].evaluate() > subpackets[1].evaluate()).into(),
                Less => (subpackets[0].evaluate() < subpackets[1].evaluate()).into(),
                Equal => (subpackets[0].evaluate() == subpackets[1].evaluate()).into(),
            },
        }
    }
}

fn parse_literal(data: &[bool]) -> (u64, usize) {
    let mut value = 0;
    let mut length = 0;
    loop {
        let group = &data[length..length + 5];
        length += 5;
        for v in &group[1..] {
            value *= 2;
            if *v {
                value += 1;
            }
        }
        if !group[0] {
            break;
        }
    }
    (value, length)
}

fn parse_inner_packets_by_count(data: &[bool]) -> (Vec<Packet>, usize) {
    const PACKET_COUNT_BITS: usize = 11;

    let sub_packet_count = get_value(&data[..PACKET_COUNT_BITS]) as usize;
    let mut sub_packets = Vec::with_capacity(sub_packet_count);
    let mut length = PACKET_COUNT_BITS;
    while sub_packets.len() < sub_packet_count {
        let new_packet = parse_packet(&data[length..]);
        length += new_packet.length;
        sub_packets.push(new_packet);
    }

    (sub_packets, length)
}

fn parse_inner_packets_by_len(data: &[bool]) -> (Vec<Packet>, usize) {
    const PACKET_LEN_BITS: usize = 15;

    let inner_len = get_value(&data[..PACKET_LEN_BITS]) as usize;
    let mut sub_packets = vec![];
    let mut length = PACKET_LEN_BITS;
    while (length - PACKET_LEN_BITS) < inner_len {
        let new_packet = parse_packet(&data[length..]);
        length += new_packet.length;
        sub_packets.push(new_packet);
    }

    (sub_packets, length)
}

fn parse_inner_packets(data: &[bool]) -> (Vec<Packet>, usize) {
    let (packets, len) = if data[0] {
        // length in sub-packets
        parse_inner_packets_by_count(&data[1..])
    } else {
        // length in bits
        parse_inner_packets_by_len(&data[1..])
    };
    (packets, len + 1)
}

fn parse_packet(data: &[bool]) -> Packet {
    let version = get_value(&data[..3]) as u8;
    let type_id = get_value(&data[3..6]) as u8;
    let mut length = 6;

    match type_id {
        4 => {
            // literal
            let (value, extra_len) = parse_literal(&data[length..]);
            length += extra_len;
            Packet {
                version,
                content: PacketContent::Literal(value),
                length,
            }
        }
        0..=7 => {
            let (sub_packets, extra_len) = parse_inner_packets(&data[length..]);
            length += extra_len;
            Packet {
                version,
                content: PacketContent::Complex(type_id.into(), sub_packets),
                length,
            }
        }
        _ => panic!("Invalid type id"),
    }
}

fn get_value(slice: &[bool]) -> u32 {
    let mut value = 0;
    for b in slice {
        value <<= 1;
        if *b {
            value |= 0b1;
        }
    }
    value
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn ascii_digit_to_bool_array(digit: &u8) -> Option<[bool; 4]> {
        ascii_digit_to_value(*digit).map(|v| {
            let mut array = [false; 4];
            for (i, elem) in array.iter_mut().enumerate() {
                if ((v >> (3 - i)) & 0b1) != 0 {
                    *elem = true;
                }
            }
            array
        })
    }

    let bits = file
        .split_last()
        .context("Failed to exclude the newline character at the end (input too short?)")?
        .1
        .iter()
        .map(ascii_digit_to_bool_array)
        .collect::<Option<Vec<_>>>()
        .map(|v| v.into_iter().flatten().collect::<Vec<_>>())
        .context("Failed to convert all digits");

    bits.map(|vec| parse_packet(&vec))
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    input.version_sum()
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    input.evaluate()
}
