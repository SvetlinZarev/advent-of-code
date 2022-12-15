use std::str::FromStr;

const PART_1_ROW: i32 = 2_000_000;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    r: i32,
    c: i32,
}

impl Point {
    pub fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    pub fn manhattan(self, other: Self) -> u32 {
        self.r.abs_diff(other.r) + self.c.abs_diff(other.c)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pair {
    sensor: Point,
    beacon: Point,
}

impl Pair {
    pub fn manhattan(self) -> u32 {
        self.sensor.manhattan(self.beacon)
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 12 {
            return Err(format!("string is too short: {}", s));
        }

        let (_, rest) = s
            .split_once('=')
            .ok_or_else(|| format!("cannot parse: {}", s))?;
        let (sensor_col, rest) = rest
            .split_once(',')
            .ok_or_else(|| format!("cannot parse: {}", s))?;
        let (_, rest) = rest
            .split_once('=')
            .ok_or_else(|| format!("cannot parse: {}", s))?;
        let (sensor_row, rest) = rest
            .split_once(':')
            .ok_or_else(|| format!("cannot parse: {}", s))?;

        let (_, rest) = rest
            .split_once('=')
            .ok_or_else(|| format!("cannot parse: {}", s))?;
        let (beacon_col, rest) = rest
            .split_once(',')
            .ok_or_else(|| format!("cannot parse: {}", s))?;
        let (_, beacon_row) = rest
            .split_once('=')
            .ok_or_else(|| format!("cannot parse: {}", s))?;

        let sensor_col = sensor_col
            .parse()
            .map_err(|_| format!("cannot parse: {}", sensor_col))?;
        let sensor_row = sensor_row
            .parse()
            .map_err(|_| format!("cannot parse: {}", sensor_row))?;
        let beacon_col = beacon_col
            .parse()
            .map_err(|_| format!("cannot parse: {}", beacon_col))?;
        let beacon_row = beacon_row
            .parse()
            .map_err(|_| format!("cannot parse: {}", beacon_row))?;

        let sensor = Point::new(sensor_row, sensor_col);
        let beacon = Point::new(beacon_row, beacon_col);

        Ok(Self { sensor, beacon })
    }
}

pub fn part_one(input: &[Pair]) -> usize {
    let mut intervals = vec![];
    let mut beacons = vec![];

    for pair in input.iter().copied() {
        if pair.beacon.r == PART_1_ROW {
            beacons.push(pair.beacon.c);
        }

        let abs_dist = PART_1_ROW.abs_diff(pair.sensor.r);
        let man_dist = pair.manhattan();

        if abs_dist > man_dist {
            continue;
        }

        // Width of the interval at row/col of the sensor
        let l = pair.sensor.c.saturating_sub_unsigned(man_dist);
        let r = pair.sensor.c.saturating_add_unsigned(man_dist);

        // Width of the interval at row/col of the target row
        let l = l.saturating_add_unsigned(abs_dist);
        let r = r.saturating_sub_unsigned(abs_dist);

        // Make the interval from the form [l;r] into the form [l; r)
        intervals.push((l, r + 1));
    }

    // Fast path: The target row is not covered by any sensors
    if intervals.is_empty() {
        return 0;
    }

    // Sort the intervals in order to be able to merge them
    intervals.sort_unstable_by_key(|&(l, r)| (l, -r));

    // dedup the beacons
    beacons.sort_unstable();
    beacons.dedup();

    // Merge the intervals and count how many cells are used
    let mut l = intervals[0].0;
    let mut r = intervals[0].1;
    let mut cells = 0;

    for (begin, end) in intervals.into_iter().skip(1) {
        if begin <= r {
            r = r.max(end);
        } else {
            cells += (r - l) as usize;

            // remove the beacons that overlap with the cells we've just counted
            let before = beacons.len();
            beacons.retain(|b| !(l..r).contains(b));
            let after = beacons.len();
            cells -= before - after;

            l = begin;
            r = end;
        }
    }

    // handle the last interval
    cells += (r - l) as usize;

    // remove the beacons that overlap with the cells we've just counted
    let before = beacons.len();
    beacons.retain(|b| !(l..r).contains(b));
    let after = beacons.len();
    cells -= before - after;

    cells
}

pub fn part_two_v1(input: &[Pair]) -> u64 {
    let mut buffer = vec![];
    let mut result = vec![];

    for row in 0..4_000_001 {
        buffer.clear();
        result.clear();

        occupied_cells(input, row, &mut buffer, &mut result);
        if result.len() != 1 {
            let r = row as u64;
            let c = result[0].1 as u64;

            return r + c * 4_000_000;
        }
    }

    unreachable!("there must be a solution")
}

pub fn occupied_cells(
    input: &[Pair],
    target: i32,
    intervals: &mut Vec<(i32, i32)>,
    merged: &mut Vec<(i32, i32)>,
) {
    for pair in input.iter().copied() {
        let abs_dist = target.abs_diff(pair.sensor.r);
        let man_dist = pair.manhattan();

        if abs_dist > man_dist {
            continue;
        }

        // Width of the interval at row/col of the sensor
        let l = pair.sensor.c.saturating_sub_unsigned(man_dist);
        let r = pair.sensor.c.saturating_add_unsigned(man_dist);

        // Width of the interval at row/col of the target row
        let l = l.saturating_add_unsigned(abs_dist);
        let r = r.saturating_sub_unsigned(abs_dist);

        // Make the interval from the form [l;r] into the form [l; r)
        intervals.push((l, r + 1));
    }

    // Fast path: The target row is not covered by any sensors
    if intervals.is_empty() {
        return;
    }

    // Sort the intervals in order to be able to merge them
    intervals.sort_unstable_by_key(|&(l, r)| (l, -r));

    // Merge the intervals and count how many cells are used
    let mut l = intervals[0].0;
    let mut r = intervals[0].1;

    for (begin, end) in intervals.iter().copied().skip(1) {
        if begin <= r {
            r = r.max(end);
        } else {
            merged.push((l, r));

            l = begin;
            r = end;
        }
    }
    merged.push((l, r));
}

// Another solution to PART-2 is to find the lines of the perimeters of each
// sensor, expanded by 1 cell. Each sensor has 4 lines (left->up, left->down,
// right->up, right->down). Then we must find 4 sensors that have the lines of
// their perimeters intersecting at the same point. The problem statement
// guarantees exactly one such point

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two_v1};
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(4_883_971, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two_v1(&input);
        assert_eq!(12_691_026_767_556, answer);
    }
}
