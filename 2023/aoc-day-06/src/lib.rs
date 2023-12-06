#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Race {
    time: u32,
    dist: u32,
}

pub fn parse_input(i: impl AsRef<str>) -> Vec<Race> {
    let i = i.as_ref();
    let (times, dists) = i.split_once('\n').unwrap();

    assert!(times.starts_with("Time:"));
    let times = &times[5..];

    assert!(dists.starts_with("Distance:"));
    let dists = &dists[9..];

    let times = times
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();
    let dists = dists
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    assert_eq!(dists.len(), times.len());
    times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(t, d)| Race { time: t, dist: d })
        .collect()
}

pub fn part_one(input: &[Race]) -> u32 {
    let mut answer = 1;

    for race in input.iter().copied() {
        let mut ways = 0;

        for t in 1..race.time {
            let d = t * (race.time - t);
            ways += (d > race.dist) as u32;
        }

        answer *= ways;
    }

    answer
}

pub fn part_two_naive(input: &[Race]) -> u64 {
    let (time, dist) = combine(input);

    let mut ways = 0;

    for t in 1..time {
        let d = t * (time - t);
        ways += (d > dist) as u64;
    }

    ways
}

pub fn part_two_naive2(input: &[Race]) -> u64 {
    let (time, dist) = combine(input);

    let mut ways = 0;
    let mut l = 1;
    let mut r = time - 1;

    // because the multiplication is fully mirrored, ie.`1*4, 2*3, 3*2, 4*1`
    // we need to process only half the range and multiply the result by 2
    while l < r {
        let d = l * r;
        ways += (d > dist) as u64;

        l += 1;
        r -= 1;
    }

    ways *= 2;

    // then we need to handle the case where the range is odd, because
    // the middle number must not be multiplied by 2
    if l == r {
        ways += (l * r > dist) as u64;
    }

    ways
}

pub fn part_two_binary_search(input: &[Race]) -> u64 {
    let (time, dist) = combine(input);

    // because the multiplication is mirrored, the max
    // distance we can travel is always at the middle of the range
    let peak = time / 2;

    // corner case - we can never beat the best record
    if peak * (time - peak) <= dist {
        return 0;
    }

    // when the "TIME-1" is even, then the peak appears twice, such as `1,2,3,3,2,1`,
    // thus we need to handle that corner case
    let correction_if_peak_appears_once = (time % 2 == 0) as u64;

    let mut lo = 0;
    let mut hi = peak;

    while lo < hi {
        let mid = (hi - lo) / 2 + lo;
        let d = mid * (time - mid);

        if d <= dist {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    (peak - lo + 1) * 2 - correction_if_peak_appears_once
}

pub fn part_two_math(input: &[Race]) -> u64 {
    let (time, dist) = combine(input);

    // it's a quadratic equation: `x * (time - x) > dist`
    // which can be simplified to : `x2 - x*time + dist < 0`
    // after we find the two roots, then the answer is the distance between them
    // formula: m,n = (-b +- SQRT(b^2 - 4*a*c)) / (2 * a)
    let a = time as f64 - ((time.pow(2) - 4 * dist) as f64).sqrt();
    let b = time as f64 + ((time.pow(2) - 4 * dist) as f64).sqrt();

    let lower_bound = (a / 2.0).floor();
    let upper_bound = (b / 2.0).ceil();

    (upper_bound - lower_bound - 1.0) as u64
}

fn combine(input: &[Race]) -> (u64, u64) {
    let mut time = 0u64;
    let mut dist = 0u64;

    for r in input.iter().copied().rev() {
        time += r.time as u64 * 10u64.pow(if time == 0 { 0 } else { time.ilog10() + 1 });
        dist += r.dist as u64 * 10u64.pow(if dist == 0 { 0 } else { dist.ilog10() + 1 });
    }
    (time, dist)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(input);

        let answer = part_one(&input);
        assert_eq!(625_968, answer);
    }

    #[test]
    fn test_part_two_naive() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(input);

        let answer = part_two_naive(&input);
        assert_eq!(43_663_323, answer);
    }

    #[test]
    fn test_part_two_naive2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(input);

        let answer = part_two_naive2(&input);
        assert_eq!(43_663_323, answer);
    }

    #[test]
    fn test_part_two_binary_search() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(input);

        let answer = part_two_binary_search(&input);
        assert_eq!(43_663_323, answer);
    }

    #[test]
    fn test_part_two_math() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(input);

        let answer = part_two_math(&input);
        assert_eq!(43_663_323, answer);
    }

    #[test]
    fn test_part_two_naive2_sanity() {
        sanity_check(part_two_naive2)
    }

    #[test]
    fn test_part_two_math_sanity() {
        sanity_check(part_two_math)
    }

    #[test]
    fn test_part_two_binary_search_sanity() {
        sanity_check(part_two_binary_search)
    }

    fn sanity_check(f: fn(&[Race]) -> u64) {
        for t in 2..250 {
            let maxd = (t / 2) * (t - t / 2);

            for d in 0..maxd {
                let test = &[Race { time: t, dist: d }];
                assert_eq!(part_two_naive(test), f(test), "Time: {}; Dist: {}", t, d);
            }
        }
    }
}
