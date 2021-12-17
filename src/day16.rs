use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum OperationType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
struct Operation {
    op_type: OperationType,
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
enum PacketValue {
    Literal(u64),
    Operation(Operation),
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    value: PacketValue,
    length: usize,
    result: u64,
}

impl Packet {
    fn evaluate(&mut self) {
        match &mut self.value {
            PacketValue::Literal(value) => self.result = *value,
            PacketValue::Operation(operation) => {
                for p in operation.subpackets.iter_mut() {
                    p.evaluate();
                }
                match operation.op_type {
                    OperationType::Sum => {
                        for p in operation.subpackets.iter() {
                            self.result += p.result;
                        }
                    }
                    OperationType::Product => {
                        self.result = 1;
                        for p in operation.subpackets.iter() {
                            self.result *= p.result;
                        }
                    }
                    OperationType::Min => {
                        self.result = u64::MAX;
                        for p in operation.subpackets.iter() {
                            self.result = self.result.min(p.result);
                        }
                    }
                    OperationType::Max => {
                        self.result = u64::MIN;
                        for p in operation.subpackets.iter() {
                            self.result = self.result.max(p.result);
                        }
                    }
                    OperationType::GreaterThan => {
                        self.result =
                            if operation.subpackets[0].result > operation.subpackets[1].result {
                                1
                            } else {
                                0
                            };
                    }
                    OperationType::LessThan => {
                        self.result =
                            if operation.subpackets[0].result < operation.subpackets[1].result {
                                1
                            } else {
                                0
                            };
                    }
                    OperationType::EqualTo => {
                        self.result =
                            if operation.subpackets[0].result == operation.subpackets[1].result {
                                1
                            } else {
                                0
                            };
                    }
                }
            }
        }
    }
}

fn read_bits(packet: &[u8], read_head: &mut usize, bit_len: usize) -> u64 {
    let mut result = 0u64;
    for i in 0..bit_len {
        let i_byte = (*read_head + i) / 8;
        let shift = 8 - ((*read_head + i) % 8) - 1;
        result <<= 1;
        result |= 0b1 & ((packet[i_byte] as u64) >> shift);
    }

    *read_head += bit_len;
    result
}

fn read_literal(packet: &[u8], read_head: &mut usize) -> u64 {
    let mut value = 0u64;

    loop {
        let group = read_bits(packet, read_head, 5);

        value <<= 4;
        value |= group & 0b1111;

        if group < 0b1_0000 {
            break;
        }
    }

    value
}

fn read_packet(packet: &[u8], read_head: &mut usize) -> Packet {
    let packet_start = *read_head;
    let version = read_bits(packet, read_head, 3);
    let type_id = read_bits(packet, read_head, 3);

    if type_id == 4 {
        return Packet {
            version,
            type_id,
            value: PacketValue::Literal(read_literal(packet, read_head)),
            length: *read_head - packet_start,
            result: 0,
        };
    }

    let mut subpackets = Vec::with_capacity(100);
    let length_type_id = read_bits(packet, read_head, 1);

    if length_type_id == 1 {
        let total_num_packets = read_bits(packet, read_head, 11);
        let mut running_packet_count = 0usize;
        while running_packet_count < total_num_packets as usize {
            let subpacket = read_packet(packet, read_head);
            running_packet_count += 1;
            subpackets.push(subpacket);
        }
    } else {
        let total_packet_length = read_bits(packet, read_head, 15);
        let mut running_packet_length = 0usize;
        while running_packet_length < total_packet_length as usize {
            let subpacket = read_packet(packet, read_head);
            running_packet_length += subpacket.length;
            subpackets.push(subpacket);
        }
    }
    let op_type = match type_id {
        0 => OperationType::Sum,
        1 => OperationType::Product,
        2 => OperationType::Min,
        3 => OperationType::Max,
        5 => OperationType::GreaterThan,
        6 => OperationType::LessThan,
        7 => OperationType::EqualTo,
        _ => {
            panic!("Invalid type id");
        }
    };

    Packet {
        version,
        type_id,
        value: PacketValue::Operation(Operation {
            subpackets,
            op_type,
        }),
        length: *read_head - packet_start,
        result: 0,
    }
}

fn count_versions(packet: &Packet) -> u64 {
    match &packet.value {
        PacketValue::Literal(_) => packet.version,
        PacketValue::Operation(operation) => {
            let mut result = packet.version;
            for subpacket in operation.subpackets.iter() {
                result += count_versions(subpacket);
            }

            result
        }
    }
}

pub fn solve(file_input: File) -> Result<(i64, i64), &'static str> {
    let input = read_input(file_input)?;

    let mut read_head = 0usize;
    let mut packet = read_packet(&input, &mut read_head);
    let part1 = count_versions(&packet);
    packet.evaluate();

    Ok((part1 as i64, packet.result as i64))
}

pub fn read_input(file_input: File) -> Result<Vec<u8>, &'static str> {
    let mut reader = BufReader::new(file_input);

    let mut input: Vec<u8> = Vec::with_capacity(1000);
    let mut byte_buf = [0u8; 2];
    while reader
        .read(&mut byte_buf)
        .map_err(|_| "Error parsing input")?
        != 0
    {
        if byte_buf[0] == '\n' as u8 {
            break;
        }

        let high = from_hex_digit(byte_buf[0]);
        let low = from_hex_digit(byte_buf[1]);
        input.push(((high << 4) | low) as u8);
    }

    Ok(input)
}

fn from_hex_digit(input: u8) -> u32 {
    match input {
        48..=57 => input as u32 - 48,
        65..=70 => input as u32 - 65 + 10,
        _ => panic!("Cannot convert from hex {}", input),
    }
}
