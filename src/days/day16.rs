use anyhow::Context;

use crate::parse::ascii_digit_to_value;

type SolverInput = Vec<bool>;

enum PacketContent {
    Literal(u64),
    Complex(Vec<Packet>),
}

struct Packet {
    version: u8,
    content: PacketContent,
    length: usize,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        if let PacketContent::Complex(inner_packets) = &self.content {
            self.version as u32 + inner_packets.iter().map(Packet::version_sum).sum::<u32>()
        } else {
            self.version as u32
        }
    }
}

fn parse_packet(data: &[bool]) -> Packet {
    let version = get_value(&data[..3]) as u8;
    let type_id = get_value(&data[3..6]);
    let mut length = 6;
    if type_id == 4 {
        // literal
        let mut value = 0;
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
        Packet {
            version,
            content: PacketContent::Literal(value),
            length,
        }
    } else {
        let mut sub_packets = vec![];
        // operator
        if data[6] {
            // length in sub-packets
            let sub_packet_count = get_value(&data[7..18]) as usize;
            length = 18;
            while sub_packets.len() < sub_packet_count {
                let new_packet = parse_packet(&data[length..]);
                length += new_packet.length;
                sub_packets.push(new_packet);
            }
        } else {
            // length in bits
            let inner_len = get_value(&data[7..22]) as usize;
            length = 22;
            while (length - 22) < inner_len {
                let new_packet = parse_packet(&data[length..]);
                length += new_packet.length;
                sub_packets.push(new_packet);
            }
        }
        Packet {
            version,
            content: PacketContent::Complex(sub_packets),
            length,
        }
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

pub fn parse_input(file: &[u8]) -> anyhow::Result<SolverInput> {
    fn ascii_digit_to_bool_array(digit: &u8) -> Option<[bool; 4]> {
        ascii_digit_to_value(*digit).map(|v| {
            let mut array = [false; 4];
            for i in 0..4 {
                if ((v >> (3 - i)) & 0b1) != 0 {
                    array[i] = true;
                }
            }
            array
        })
    }

    file.split_last()
        .context("Failed to exclude the newline character at the end (input too short?)")?
        .1
        .into_iter()
        .map(ascii_digit_to_bool_array)
        .collect::<Option<Vec<_>>>()
        .map(|v| v.into_iter().flatten().collect())
        .context("Failed to convert all digits")
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    parse_packet(input).version_sum()
}
