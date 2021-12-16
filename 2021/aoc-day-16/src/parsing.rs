pub fn parse_to_binary<S: AsRef<str>>(src: S) -> (Vec<u8>, usize) {
    to_bit_vec(src.as_ref().trim_end().as_bytes())
}

fn to_bit_vec(src: &[u8]) -> (Vec<u8>, usize) {
    let mut result = Vec::with_capacity((src.len() + 1) / 2);

    for idx in (0..src.len() - 1).step_by(2) {
        result.push((to_binary(src[idx]) << 4) | (to_binary(src[idx + 1])));
    }

    let mut limit = result.len() * 8;
    if src.len() & 1 != 0 {
        result.push(to_binary(src[src.len() - 1]) << 4);
        limit += 4;
    }

    (result, limit)
}

fn to_binary(x: u8) -> u8 {
    match x {
        b'0'..=b'9' => x - b'0',
        _ => x - b'A' + 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_even_hex_string() {
        let (bits, limit) = to_bit_vec("D2FE28".as_bytes());
        assert_eq!(vec![(13u8 << 4) | 2u8, (15u8 << 4) | 14u8, (2u8 << 4) | 8u8], bits);
        assert_eq!(24, limit);
    }

    #[test]
    fn parse_odd_hex_string() {
        let (bits, limit) = to_bit_vec("D2FE28A".as_bytes());
        assert_eq!(vec![(13u8 << 4) | 2u8, (15u8 << 4) | 14u8, (2u8 << 4) | 8u8, 10u8 << 4], bits);
        assert_eq!(28, limit);

    }
}
