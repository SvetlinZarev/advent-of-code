pub fn solve_bruteforce(input: &[i32]) -> Option<i32> {
    for (idx, &a) in input.iter().enumerate() {
        for &b in input.iter().skip(idx + 1) {
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }

    None
}

pub fn solve_with_sorting(input: &[i32]) -> Option<i32> {
    let mut input = input.to_vec();
    input.sort_unstable();

    let mut l = 0;
    let mut r = input.len() - 1;

    while l < r {
        let lv = input[l];
        let rv = input[r];
        let sum = lv + rv;

        if sum == 2020 {
            return Some(lv * rv);
        } else if sum < 2020 {
            l += 1;
        } else {
            r -= 1;
        }
    }

    None
}
