use crate::packets::{Content, Packet};

pub fn part_one(packet: &Packet) -> u32 {
    dfs(packet)
}

fn dfs(packet: &Packet) -> u32 {
    if let Content::Operator(_, packets) = packet.content() {
        return packets.iter().fold(packet.version(), |sum, p| sum + dfs(p));
    }

    packet.version()
}


#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use crate::{decode_packets, parse_to_binary};
    use super::*;

    #[test]
    fn test_part_one() {
        let (binary_input, bits) = parse_to_binary(load_text_input_from_file("inputs/input.txt"));
        let decoded_input = decode_packets(&binary_input, bits);

        let answer = part_one(&decoded_input);
        assert_eq!(860, answer);
    }
}