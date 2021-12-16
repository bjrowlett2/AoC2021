mod aoc;

use std::collections::HashMap;

struct Day16 {
    binary: String,
}

const PACKET_SUM: i64 = 0;
const PACKET_PRODUCT: i64 = 1;
const PACKET_MINIMUM: i64 = 2;
const PACKET_MAXIMUM: i64 = 3;
const PACKET_LITERAL: i64 = 4;
const PACKET_GREATER_THAN: i64 = 5;
const PACKET_LESS_THAN: i64 = 6;
const PACKET_EQUAL_TO: i64 = 7;

struct Packet {
    version: i64,
    type_id: i64,
    literal: Option<i64>,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new() -> Packet {
        return Packet {
            version: 0,
            type_id: 0,
            literal: None,
            sub_packets: Vec::new(),
        };
    }

    fn checksum(&self) -> i64 {
        let mut total = self.version;
        for sub_packet in &self.sub_packets {
            total += sub_packet.checksum();
        }

        return total;
    }

    fn sum(packets: &Vec<Packet>) -> i64 {
        let mut result = 0;
        for packet in packets {
            result += packet.evaluate();
        }

        return result;
    }

    fn product(packets: &Vec<Packet>) -> i64 {
        let mut result = 1;
        for packet in packets {
            result *= packet.evaluate();
        }

        return result;
    }

    fn minimum(packets: &Vec<Packet>) -> i64 {
        let mut result = i64::MAX;
        for packet in packets {
            let value = packet.evaluate();
            if value < result { result = value; }
        }

        return result;
    }

    fn maximum(packets: &Vec<Packet>) -> i64 {
        let mut result = i64::MIN;
        for packet in packets {
            let value = packet.evaluate();
            if value > result { result = value; }
        }

        return result;
    }

    fn equal_to(packets: &Vec<Packet>) -> i64 {
        let first = packets[0].evaluate();
        for i in 1..packets.len() {
            if first != packets[i].evaluate() { return 0; }
        }

        return 1;
    }

    fn less_than(packets: &Vec<Packet>) -> i64 {
        let first = packets[0].evaluate();
        for i in 1..packets.len() {
            if first >= packets[i].evaluate() { return 0; }
        }

        return 1;
    }

    fn greater_than(packets: &Vec<Packet>) -> i64 {
        let first = packets[0].evaluate();
        for i in 1..packets.len() {
            if first <= packets[i].evaluate() { return 0; }
        }

        return 1;
    }

    fn evaluate(&self) -> i64 {
        return match self.type_id {
            PACKET_LITERAL      => self.literal.unwrap(),
            PACKET_SUM          => Packet::sum(&self.sub_packets),
            PACKET_PRODUCT      => Packet::product(&self.sub_packets),
            PACKET_MINIMUM      => Packet::minimum(&self.sub_packets),
            PACKET_MAXIMUM      => Packet::maximum(&self.sub_packets),
            PACKET_EQUAL_TO     => Packet::equal_to(&self.sub_packets),
            PACKET_LESS_THAN    => Packet::less_than(&self.sub_packets),
            PACKET_GREATER_THAN => Packet::greater_than(&self.sub_packets),
            unknown => panic!("Unknown packet type id: {}", unknown),
        };
    }
}

fn binary_to_decimal(binary: &str) -> i64 {
    let mut value = 0;
    for ch in binary.chars() {
        value = value << 1;
        if ch == '1' { value += 1; }
    }

    return value;
}

fn hexadecimal_to_binary(ch: char) -> String {
    let symbols = HashMap::from([
        ('0', "0000"), ('1', "0001"), ('2', "0010"), ('3', "0011"),
        ('4', "0100"), ('5', "0101"), ('6', "0110"), ('7', "0111"),
        ('8', "1000"), ('9', "1001"), ('A', "1010"), ('B', "1011"),
        ('C', "1100"), ('D', "1101"), ('E', "1110"), ('F', "1111"),
    ]);

    return match symbols.get(&ch) {
        Some(value) => value.to_string(),
        None => panic!("No hex symbol found"),
    };
}

fn read_literal(binary: &String, offset: usize) -> (usize, i64) {
    let mut i = offset;
    let mut value = String::new();

    loop {
        let slice_range = i..(i + 5);
        let slice = &binary[slice_range];

        i += 5;
        value.push_str(&slice[1..5]);
        if slice.starts_with('0') { break; }
    }

    return (i - offset, binary_to_decimal(&value.as_str()));
}

fn read_packet(binary: &String, offset: usize) -> (usize, Packet) {
    let mut consumed = 0;
    let mut packet = Packet::new();

    let version_range = offset..(offset + 3);
    packet.version = binary_to_decimal(&binary[version_range]);

    let type_id_range = (offset + 3)..(offset + 6);
    packet.type_id = binary_to_decimal(&binary[type_id_range]);

    consumed += 6;
    if packet.type_id == PACKET_LITERAL {
        let literal = read_literal(binary, offset + 6);

        consumed += literal.0;
        packet.literal = Some(literal.1);
    } else {
        let length_type_id_range = (offset + 6)..(offset + 7);
        let length_type_id = binary_to_decimal(&binary[length_type_id_range]);

        consumed += 1;
        if length_type_id == 0 {
            let bits_in_sub_packet_range = (offset + 7)..(offset + 22);
            let bits_in_sub_packet = binary_to_decimal(&binary[bits_in_sub_packet_range]);

            consumed += 15;
            let mut processed = 0;
            while processed < (bits_in_sub_packet as usize) {
                let sub_packet_offset = offset + consumed + processed;
                let sub_packet = read_packet(binary, sub_packet_offset);

                processed += sub_packet.0;
                packet.sub_packets.push(sub_packet.1);
            }

            consumed += processed;
        } else if length_type_id == 1 {
            let num_sub_packets_range = (offset + 7)..(offset + 18);
            let num_sub_packets = binary_to_decimal(&binary[num_sub_packets_range]);

            consumed += 11;
            for _ in 0..num_sub_packets {
                let sub_packet_offset = offset + consumed;
                let subpacket = read_packet(binary, sub_packet_offset);

                consumed += subpacket.0;
                packet.sub_packets.push(subpacket.1);
            }
        }
    }

    return (consumed, packet);
}

fn main() {
    let mut day = Day16 {
        binary: String::new(),
    };

    for line in aoc::lines("inputs/day_16.txt") {
        for ch in line.chars() {
            day.binary.push_str(&hexadecimal_to_binary(ch));
        }
    }

    match solve_part_1(&day) {
        Ok(value) => println!("Part 1: {}", value),
        Err(reason) => panic!("solve_part_1 failed: {}", reason),
    };

    match solve_part_2(&day) {
        Ok(value) => println!("Part 2: {}", value),
        Err(reason) => panic!("solve_part_2 failed: {}", reason),
    };
}

fn solve_part_1(day: &Day16) -> Result<i64, String> {
    let (_, packet) = read_packet(&day.binary, 0);
    return Ok(packet.checksum());
}

fn solve_part_2(day: &Day16) -> Result<i64, String> {
    let (_, packet) = read_packet(&day.binary, 0);
    return Ok(packet.evaluate());
}
