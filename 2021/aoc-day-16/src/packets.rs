use std::cmp::Ordering;

#[derive(Debug)]
struct BitStream<'a> {
    bits: &'a [u8],
    next: usize,
    limit: usize,
}

impl<'a> BitStream<'a> {
    pub fn new(src: &'a [u8], limit: usize) -> Self {
        Self { bits: src, next: 0, limit }
    }

    pub fn read(&mut self, bits: u8) -> u8 {
        debug_assert!(bits <= 8);

        let idx = self.next / 8;
        let skip = self.next % 8;

        let mut result = (self.bits[idx] << skip) >> skip;
        match bits.cmp(&(8 - skip as u8)) {
            Ordering::Less => { // we have read too many bits.
                result = result >> (8u8 - skip as u8 - bits);
            }
            Ordering::Greater => {// we have read too few bits.
                let additional = self.bits[idx + 1];
                let to_read = bits - (8 - skip as u8);
                result = (result << to_read) | (additional >> (8 - to_read));
            }
            Ordering::Equal => { /*no-op*/ }
        }

        self.next += bits as usize;
        result
    }

    #[cfg(test)]
    pub fn has_more(&self) -> bool {
        self.next < self.limit
    }

    pub fn remaining(&self) -> usize {
        self.limit - self.next
    }

    pub fn sub_stream(&mut self, limit: usize) -> Self {
        assert!(self.next + limit <= self.limit);

        let before_skip = self.next;
        self.next += limit;

        BitStream {
            bits: self.bits,
            next: before_skip,
            limit: self.next,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Packet {
    version: u32,
    content: Content,
}

impl Packet {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Content {
    Literal(u64),
    Operator(u8, Vec<Packet>),
}

pub fn decode_packets(input: &[u8], bits: usize) -> Packet {
    let mut src = BitStream::new(input, bits);
    let mut packets = decode_internal(&mut src, 1);

    assert_eq!(1, packets.len(), "{:#?}", packets);
    packets.remove(0)
}

fn decode_internal(src: &mut BitStream, max_packets: u16) -> Vec<Packet> {
    let mut packets = vec![];

    while src.remaining() > 6 && packets.len() < max_packets as usize {
        let version = src.read(3) as u32;
        let kind = src.read(3);

        let content = match kind {
            4 => decode_literal(src),
            _ => decode_operator(src, kind)
        };

        packets.push(Packet {
            version,
            content,
        });
    }

    packets
}

fn decode_literal(src: &mut BitStream) -> Content {
    let mut number = 0;

    // arbitrarily assume that the arbitrarily long numbers are at most 64bits
    loop {
        let chunk = src.read(5);
        number <<= 4;
        number |= ((chunk << 4) >> 4) as u64;

        if chunk & 0b0001_0000 == 0 {
            break;
        }
    }

    Content::Literal(number)
}

fn decode_operator(src: &mut BitStream, id: u8) -> Content {
    let mode = src.read(1);
    if mode == 0 {
        decode_operator_0(src, id)
    } else {
        decode_operator_1(src, id)
    }
}

fn decode_operator_0(src: &mut BitStream, id: u8) -> Content {
    let sub_packet_bits = ((src.read(8) as u16) << 7) | (src.read(7) as u16);
    Content::Operator(id, decode_internal(&mut src.sub_stream(sub_packet_bits as usize), u16::MAX))
}

fn decode_operator_1(src: &mut BitStream, id: u8) -> Content {
    let sub_packets = ((src.read(8) as u16) << 3) | (src.read(3) as u16);
    Content::Operator(id, decode_internal(src, sub_packets))
}

#[cfg(test)]
mod tests {
    use crate::parse_to_binary;
    use super::*;

    #[test]
    fn test_read_bits_from_single_byte() {
        let data = vec![0b10101110];
        let mut stream = BitStream::new(&data, 8);

        assert_eq!(0b00000001, stream.read(1));
        assert!(stream.has_more());

        assert_eq!(0b00000000, stream.read(1));
        assert!(stream.has_more());

        assert_eq!(0b00000001, stream.read(1));
        assert!(stream.has_more());

        assert_eq!(0b00000000, stream.read(1));
        assert!(stream.has_more());

        assert_eq!(0b00000011, stream.read(2));
        assert!(stream.has_more());

        assert_eq!(0b00000010, stream.read(2));
        assert!(!stream.has_more());
    }

    #[test]
    fn test_read_bits_from_multiple_bytes() {
        let data = vec![0b11100011, 0b10001110];
        let mut stream = BitStream::new(&data, 16);

        assert_eq!(0b00000111, stream.read(3));
        assert!(stream.has_more());

        assert_eq!(0b00000000, stream.read(3));
        assert!(stream.has_more());

        assert_eq!(0b00000111, stream.read(3));
        assert!(stream.has_more());

        assert_eq!(0b00000000, stream.read(3));
        assert!(stream.has_more());

        assert_eq!(0b00000111, stream.read(3));
        assert!(stream.has_more());

        assert_eq!(0b00000000, stream.read(1));
        assert!(!stream.has_more());
    }

    #[test]
    fn test_decode_literal_packet() {
        let (data, len) = parse_to_binary("D2FE28");
        let packet = decode_packets(&data, len);

        assert_eq!(Packet {
            version: 6,
            content: Content::Literal(2021),
        }, packet);
    }

    #[test]
    fn test_decode_operator_packet_0() {
        let (data, len) = parse_to_binary("38006F45291200");
        let packet = decode_packets(&data, len);

        let expected = Packet {
            version: 1,
            content: Content::Operator(6, vec![
                Packet {
                    version: 6,
                    content: Content::Literal(10),
                },
                Packet {
                    version: 2,
                    content: Content::Literal(20),
                },
            ]),
        };

        assert_eq!(expected, packet, "{:#?}", packet);
    }

    #[test]
    fn test_decode_operator_packet_1() {
        let (data, len) = parse_to_binary("EE00D40C823060");
        let packet = decode_packets(&data, len);

        let expected = Packet {
            version: 7,
            content: Content::Operator(3, vec![
                Packet {
                    version: 2,
                    content: Content::Literal(1),
                },
                Packet {
                    version: 4,
                    content: Content::Literal(2),
                },
                Packet {
                    version: 1,
                    content: Content::Literal(3),
                },
            ]),
        };

        assert_eq!(expected, packet, "{:#?}", packet);
    }
}
