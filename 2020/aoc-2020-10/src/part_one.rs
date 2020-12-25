pub fn solve_v1(input: &mut [usize]) -> Option<usize> {
    input.sort_unstable();

    let mut ones = 0;
    let mut threes = 1; // start from one,because our adapter is always +3 jolts difference
    let mut prev = 0;

    for &i in input.iter() {
        let diff = i - prev;
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }

        prev = i;
    }

    Some(ones * threes)
}

pub fn solve_v2(input: &mut [usize]) -> Option<usize> {
    input.sort_unstable();

    // the 'threes' start from one,because our adapter is always +3 jolts difference
    // [ignored, ones, twos, threes]
    let mut sums = [0, 0, 0, 1];
    let mut prev = 0;

    for &i in input.iter() {
        let diff = i - prev;
        if diff <= 3 {
            // by definition, there are no repeating numbers,
            // so the min diff is 1 -> i.e. it's not possible to
            // go out of bounds
            sums[diff] += 1;
            prev = i;
        }
    }

    Some(sums[1] * sums[3])
}
