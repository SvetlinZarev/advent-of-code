pub fn solve_with_bruteforce(input: &[i32]) -> Option<i32> {
    for (idx_a, &a) in input.iter().enumerate() {
        for (idx_b, &b) in input.iter().enumerate().skip(idx_a) {
            for (_, &c) in input.iter().enumerate().skip(idx_b) {
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}

pub fn solve_with_quadratic_alg(input: &mut [i32]) -> Option<i32> {
    input.sort_unstable();

    for x in 0..input.len() - 3 {
        let xv = input[x];

        let mut l = x + 1;
        let mut r = input.len() - 1;

        while l < r {
            let lv = input[l];
            let rv = input[r];
            let sum = lv + rv + xv;

            if sum == 2020 {
                return Some(lv * rv * xv);
            } else if sum < 2020 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    None
}
