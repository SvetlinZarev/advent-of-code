pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

pub fn lcm(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return None; // LCM is undefined if either number is 0
    }

    Some((a * b) / gcd(a, b))
}
