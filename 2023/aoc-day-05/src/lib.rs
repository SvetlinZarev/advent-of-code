use std::collections::BTreeMap;
use std::str::Lines;

#[derive(Default)]
pub struct Input {
    seeds: Vec<u64>,
    seed_to_soil: BTreeMap<u64, (u64, u64)>,
    soil_to_fertilizer: BTreeMap<u64, (u64, u64)>,
    fertilizer_to_water: BTreeMap<u64, (u64, u64)>,
    water_to_light: BTreeMap<u64, (u64, u64)>,
    light_to_temp: BTreeMap<u64, (u64, u64)>,
    temp_to_humidity: BTreeMap<u64, (u64, u64)>,
    humidity_to_location: BTreeMap<u64, (u64, u64)>,
}

pub fn parse_input(input: impl AsRef<str>) -> Input {
    let input = input.as_ref();
    let mut lines = input.lines();

    return Input {
        seeds: read_seeds(&mut lines),
        seed_to_soil: read_seed_to_soil(&mut lines),
        soil_to_fertilizer: read_soil_to_fertilizer(&mut lines),
        fertilizer_to_water: read_fertilizer_to_water(&mut lines),
        water_to_light: read_water_to_light(&mut lines),
        light_to_temp: read_light_to_temp(&mut lines),
        temp_to_humidity: read_temp_to_humidity(&mut lines),
        humidity_to_location: read_humidity_to_location(&mut lines),
    };
}

fn read_seeds(lines: &mut Lines) -> Vec<u64> {
    let seeds_line = lines.next().unwrap();
    assert!(seeds_line.starts_with("seeds: "));

    seeds_line[7..]
        .split_ascii_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn read_seed_to_soil(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    read_label(lines, "seed-to-soil map:");
    read_map(lines)
}

fn read_soil_to_fertilizer(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    read_label(lines, "soil-to-fertilizer map:");
    read_map(lines)
}
fn read_fertilizer_to_water(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    read_label(lines, "fertilizer-to-water map:");
    read_map(lines)
}

fn read_water_to_light(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    read_label(lines, "water-to-light map:");
    read_map(lines)
}

fn read_light_to_temp(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    read_label(lines, "light-to-temperature map:");
    read_map(lines)
}

fn read_temp_to_humidity(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    read_label(lines, "temperature-to-humidity map:");
    read_map(lines)
}
fn read_humidity_to_location(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    read_label(lines, "humidity-to-location map:");
    read_map(lines)
}

fn read_label(lines: &mut Lines, label: &str) {
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        assert_eq!(label, line.trim());
        return;
    }

    panic!("end of input")
}

fn read_map(lines: &mut Lines) -> BTreeMap<u64, (u64, u64)> {
    let mut map = BTreeMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            return map;
        }

        let (dest, rest) = line.split_once(' ').unwrap();
        let (src, len) = rest.split_once(' ').unwrap();

        map.insert(
            src.trim().parse().unwrap(),
            (dest.trim().parse().unwrap(), len.trim().parse().unwrap()),
        );
    }

    map
}

pub fn part_one(input: &Input) -> u64 {
    let mut answer = u64::MAX;
    for seed in input.seeds.iter().copied() {
        let soil = next_value(seed, &input.seed_to_soil);
        let fertilizer = next_value(soil, &input.soil_to_fertilizer);
        let water = next_value(fertilizer, &input.fertilizer_to_water);
        let light = next_value(water, &input.water_to_light);
        let temp = next_value(light, &input.light_to_temp);
        let humidity = next_value(temp, &input.temp_to_humidity);
        let location = next_value(humidity, &input.humidity_to_location);

        answer = answer.min(location);
    }

    answer
}

fn next_value(key: u64, src: &BTreeMap<u64, (u64, u64)>) -> u64 {
    if let Some((&src_range, &(dst_range, range_len))) = src.range(..=key).last() {
        if key < src_range + range_len {
            return remap(key, src_range, dst_range);
        }
    }

    key
}

pub fn part_two_v1(input: &Input) -> u64 {
    let mut answer = u64::MAX;

    for seeds in input.seeds.chunks(2) {
        let soil = calc_range(&[(seeds[0], seeds[0] + seeds[1])], &input.seed_to_soil);
        let fertilizer = calc_range(&soil, &input.soil_to_fertilizer);
        let water = calc_range(&fertilizer, &input.fertilizer_to_water);
        let light = calc_range(&water, &input.water_to_light);
        let temp = calc_range(&light, &input.light_to_temp);
        let humidity = calc_range(&temp, &input.temp_to_humidity);
        let location = calc_range(&humidity, &input.humidity_to_location);

        answer = location
            .into_iter()
            .map(|x| x.0)
            .fold(answer, |acc, val| acc.min(val));
    }

    answer
}

fn calc_range(keys: &[(u64, u64)], src: &BTreeMap<u64, (u64, u64)>) -> Vec<(u64, u64)> {
    let mut result = vec![];

    for (start, end) in keys.iter().copied() {
        let mut end = end;
        while end > start {
            match src.range(..end).last() {
                None => {
                    result.push((start, end));
                    break;
                }

                Some((&src_range, &(dst_range, range_len))) => {
                    // in case the ranges are not overlapping at all
                    if src_range + range_len <= start {
                        result.push((start, end));
                        break;
                    }

                    // right part outside of src-range
                    if src_range + range_len < end {
                        result.push((src_range + range_len, end));
                        end = src_range + range_len;
                    }

                    // overlapping part
                    let begin = start.max(src_range);
                    result.push((
                        remap(begin, src_range, dst_range),
                        remap(end, src_range, dst_range),
                    ));
                    end = begin;
                }
            }
        }
    }

    result
}

pub fn part_two_v2(input: &Input) -> u64 {
    let mut answer = u64::MAX;

    for seeds in input.seeds.chunks(2) {
        let key = (seeds[0], seeds[0] + seeds[1]);
        remap_range(key, &input.seed_to_soil, |key| {
            remap_range(key, &input.soil_to_fertilizer, |key| {
                remap_range(key, &input.fertilizer_to_water, |key| {
                    remap_range(key, &input.water_to_light, |key| {
                        remap_range(key, &input.light_to_temp, |key| {
                            remap_range(key, &input.temp_to_humidity, |key| {
                                remap_range(key, &input.humidity_to_location, |key| {
                                    answer = answer.min(key.0);
                                })
                            })
                        })
                    })
                })
            })
        });
    }

    answer
}

fn remap_range(
    key: (u64, u64),
    src: &BTreeMap<u64, (u64, u64)>,
    mut consume: impl FnMut((u64, u64)),
) {
    let (start, mut end) = key;

    while end > start {
        match src.range(..end).last() {
            None => {
                consume((start, end));
                break;
            }

            Some((&src_range, &(dst_range, range_len))) => {
                // in case the ranges are not overlapping at all
                if src_range + range_len <= start {
                    consume((start, end));
                    break;
                }

                // right part outside of src-range
                if src_range + range_len < end {
                    consume((src_range + range_len, end));
                    end = src_range + range_len;
                }

                // overlapping part
                let begin = start.max(src_range);
                consume((
                    remap(begin, src_range, dst_range),
                    remap(end, src_range, dst_range),
                ));
                end = begin;
            }
        }
    }
}

fn remap(key: u64, src: u64, dst: u64) -> u64 {
    dst + (key - src)
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
        assert_eq!(551_761_867, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(input);

        let answer = part_two_v1(&input);
        assert_eq!(57_451_709, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(input);

        let answer = part_two_v2(&input);
        assert_eq!(57_451_709, answer);
    }
}
