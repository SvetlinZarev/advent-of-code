use crate::packets::{Content, Packet};

pub fn part_two(packet: &Packet) -> u64 {
    eval(packet)
}

fn eval(packet: &Packet) -> u64 {
    match packet.content() {
        Content::Literal(val) => *val,
        Content::Operator(id, packets) => {
            match id {
                0 => {
                    let mut sum = 0;
                    for p in packets.iter() {
                        sum += eval(p);
                    }
                    sum
                }

                1 => {
                    let mut prod = 1;
                    for p in packets.iter() {
                        prod *= eval(p);
                    }
                    prod
                }

                2 => {
                    if packets.is_empty() {
                        return 0;
                    }

                    let mut min = u64::MAX;
                    for p in packets.iter() {
                        min = min.min(eval(p));
                    }
                    min
                }

                3 => {
                    if packets.is_empty() {
                        return 0;
                    }

                    let mut max = u64::MIN;
                    for p in packets.iter() {
                        max = max.max(eval(p));
                    }
                    max
                }

                5 => {
                    assert_eq!(2, packets.len());
                    (eval(&packets[0]) > eval(&packets[1])) as u64
                }

                6 => {
                    assert_eq!(2, packets.len());
                    (eval(&packets[0]) < eval(&packets[1])) as u64
                }

                7 => {
                    assert_eq!(2, packets.len());
                    (eval(&packets[0]) == eval(&packets[1])) as u64
                }

                _ => panic!("Invalid packet: {:#?}", packet)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;
    use crate::{decode_packets, parse_to_binary};
    use super::*;

    #[test]
    fn test_sample_sum() {
        let (binary, bits) = parse_to_binary("C200B40A82");
        let packet = decode_packets(&binary, bits);

        // 1 + 2 == 3
        let answer = eval(&packet);
        assert_eq!(3, answer);
    }

    #[test]
    fn test_sample_product() {
        let (binary, bits) = parse_to_binary("04005AC33890");
        let packet = decode_packets(&binary, bits);

        // 6 * 9 == 54
        let answer = eval(&packet);
        assert_eq!(54, answer);
    }

    #[test]
    fn test_sample_minimum() {
        let (binary, bits) = parse_to_binary("880086C3E88112");
        let packet = decode_packets(&binary, bits);

        // min(7,8,9) == 7
        let answer = eval(&packet);
        assert_eq!(7, answer);
    }

    #[test]
    fn test_sample_maximum() {
        let (binary, bits) = parse_to_binary("880086C3E88112");
        let packet = decode_packets(&binary, bits);

        // max(7,8,9) == 9
        let answer = eval(&packet);
        assert_eq!(7, answer);
    }

    #[test]
    fn test_sample_less_than() {
        let (binary, bits) = parse_to_binary("D8005AC2A8F0");
        let packet = decode_packets(&binary, bits);

        // 1 < 15
        let answer = eval(&packet);
        assert_eq!(1, answer);
    }

    #[test]
    fn test_sample_more_than() {
        let (binary, bits) = parse_to_binary("F600BC2D8F");
        let packet = decode_packets(&binary, bits);

        // 5 < 15
        let answer = eval(&packet);
        assert_eq!(0, answer);
    }

    #[test]
    fn test_sample_equal() {
        let (binary, bits) = parse_to_binary("9C005AC2F8F0");
        let packet = decode_packets(&binary, bits);

        // 5 != 15
        let answer = eval(&packet);
        assert_eq!(0, answer);
    }

    #[test]
    fn test_sample_expression() {
        let (binary, bits) = parse_to_binary("9C0141080250320F1802104A08");
        let packet = decode_packets(&binary, bits);

        // 5 != 15
        let answer = eval(&packet);
        assert_eq!(1, answer);
    }

    #[test]
    fn test_part_two() {
        let (binary, bits) = parse_to_binary(load_text_input_from_file("inputs/input.txt"));
        let packet = decode_packets(&binary, bits);

        let answer = eval(&packet);
        assert_eq!(470949537659, answer);
    }
}