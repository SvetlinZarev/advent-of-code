use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::sync::LazyLock;

static REGEX_PARSE_BEGIN_SHIFT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\[(?<Y>\d{4})-(?<M>\d{2})-(?<D>\d{2}) (?<h>\d{2}):(?<m>\d{2})\] Guard #(?<ID>\d+) begins shift$"#)
        .unwrap()
});

static REGEX_PARSE_FALLS_ASLEEP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\[(?<Y>\d{4})-(?<M>\d{2})-(?<D>\d{2}) (?<h>\d{2}):(?<m>\d{2})\] falls asleep$"#)
        .unwrap()
});

static REGEX_PARSE_WAKES_UP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\[(?<Y>\d{4})-(?<M>\d{2})-(?<D>\d{2}) (?<h>\d{2}):(?<m>\d{2})\] wakes up$"#)
        .unwrap()
});

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
}

impl Timestamp {
    pub fn to_instant(self) -> u64 {
        self.year as u64 * 365 * 24 * 60
            + match self.month {
                1 => 0,
                2 => 31,
                3 => 31 + 28,
                4 => 31 + 28 + 31,
                5 => 31 + 28 + 31 + 30,
                6 => 31 + 28 + 31 + 30 + 31,
                7 => 31 + 28 + 31 + 30 + 31 + 30,
                8 => 31 + 28 + 31 + 30 + 31 + 30 + 31,
                9 => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
                10 => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
                11 => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
                12 => 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 30,
                _ => unreachable!(),
            } * (24 * 60)
            + self.day as u64 * 24 * 60
            + self.hour as u64 * 60
            + self.minute as u64
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Kind {
    WakeUp,
    FallAsleep,
    BeginShift(u16),
}

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub timestamp: Timestamp,
    pub kind: Kind,
}

pub fn parse_input(input: &str) -> Result<Vec<Event>, Box<dyn Error>> {
    let mut events: Vec<Event> = Vec::new();

    for line in input.lines() {
        let event = if let Some(captures) = REGEX_PARSE_WAKES_UP.captures(line) {
            let timestamp = extract_timestamp(&captures)?;
            Event {
                timestamp,
                kind: Kind::WakeUp,
            }
        } else if let Some(captures) = REGEX_PARSE_FALLS_ASLEEP.captures(line) {
            let timestamp = extract_timestamp(&captures)?;
            Event {
                timestamp,
                kind: Kind::FallAsleep,
            }
        } else if let Some(captures) = REGEX_PARSE_BEGIN_SHIFT.captures(line) {
            let timestamp = extract_timestamp(&captures)?;
            let guard_id = extract_guard_id(&captures)?;
            Event {
                timestamp,
                kind: Kind::BeginShift(guard_id),
            }
        } else {
            return Err("Year does not exists".into());
        };

        events.push(event);
    }

    events.sort_unstable_by_key(|e| e.timestamp);
    Ok(events)
}

fn extract_timestamp(captures: &regex::Captures<'_>) -> Result<Timestamp, Box<dyn Error>> {
    let Some(year) = captures.name("Y") else {
        return Err("Year does not exists".into());
    };

    let Some(month) = captures.name("M") else {
        return Err("Month does not exists".into());
    };

    let Some(day) = captures.name("D") else {
        return Err("Day does not exists".into());
    };

    let Some(hour) = captures.name("h") else {
        return Err("Hour does not exists".into());
    };

    let Some(minute) = captures.name("m") else {
        return Err("Minute does not exists".into());
    };

    Ok(Timestamp {
        year: year.as_str().parse()?,
        month: month.as_str().parse()?,
        day: day.as_str().parse()?,
        hour: hour.as_str().parse()?,
        minute: minute.as_str().parse()?,
    })
}

fn extract_guard_id(captures: &regex::Captures<'_>) -> Result<u16, Box<dyn Error>> {
    let Some(id) = captures.name("ID") else {
        return Err("Guard ID does not exists".into());
    };

    Ok(id.as_str().parse()?)
}

pub fn part_one(input: &[Event]) -> u64 {
    let mut current_guard_id = 0;
    let mut last_timestamp = Timestamp::default();
    let mut guard_sleep_durations = HashMap::new();

    for event in input {
        match event.kind {
            Kind::WakeUp => {
                let (duration, histogram) = guard_sleep_durations
                    .entry(current_guard_id)
                    .or_insert((0, [0u64; 60]));

                let a = event.timestamp.to_instant();
                let b = last_timestamp.to_instant();
                let sleep_duration = a - b;

                let histogram_base_diff = sleep_duration / 60;
                let histogram_minutes_to_update = (sleep_duration % 60) as u16;

                *duration += sleep_duration;
                if histogram_base_diff > 0 {
                    histogram.iter_mut().for_each(|i| *i += histogram_base_diff);
                }

                for diff in 0..histogram_minutes_to_update {
                    let idx = (60 + event.timestamp.minute - diff - 1) % 60;
                    histogram[idx as usize] += 1;
                }
            }

            Kind::FallAsleep => {
                //
            }

            Kind::BeginShift(guard) => {
                current_guard_id = guard;
            }
        }

        last_timestamp = event.timestamp;
    }

    let mut most_sleepy_guard = 0;
    let mut max_sleep_time = 0;
    let mut histogram = [0u64; 60];

    for (guard_id, (duration, hist)) in guard_sleep_durations {
        if duration > max_sleep_time {
            max_sleep_time = duration;
            most_sleepy_guard = guard_id;
            histogram = hist;
        }
    }

    let mut most_freq_minute = 0;
    let mut minute_frequency = 0;
    for (idx, &freq) in histogram.iter().enumerate() {
        if freq > minute_frequency {
            minute_frequency = freq;
            most_freq_minute = idx;
        }
    }

    most_sleepy_guard as u64 * most_freq_minute as u64
}

pub fn part_two(input: &Vec<Event>) -> u64 {
    let mut current_guard_id = 0;
    let mut last_timestamp = Timestamp::default();
    let mut guard_sleep_durations = HashMap::new();

    for event in input {
        match event.kind {
            Kind::WakeUp => {
                let (most_freq, histogram) = guard_sleep_durations
                    .entry(current_guard_id)
                    .or_insert((0, [0u64; 60]));

                let a = event.timestamp.to_instant();
                let b = last_timestamp.to_instant();
                let sleep_duration = a - b;

                let histogram_base_diff = sleep_duration / 60;
                let histogram_minutes_to_update = (sleep_duration % 60) as u16;

                if histogram_base_diff > 0 {
                    histogram.iter_mut().for_each(|i| *i += histogram_base_diff);
                }

                for diff in 0..histogram_minutes_to_update {
                    let idx = (60 + event.timestamp.minute - diff - 1) % 60;
                    histogram[idx as usize] += 1;

                    if histogram[idx as usize] > *most_freq {
                        *most_freq = histogram[idx as usize];
                    }
                }
            }

            Kind::FallAsleep => {
                //
            }

            Kind::BeginShift(guard) => {
                current_guard_id = guard;
            }
        }

        last_timestamp = event.timestamp;
    }

    let mut target_guard_id = 0;
    let mut highest_frequency = 0;
    let mut histogram = [0u64; 60];

    for (guard_id, (frequency, hist)) in guard_sleep_durations {
        if frequency > highest_frequency {
            highest_frequency = frequency;
            target_guard_id = guard_id;
            histogram = hist;
        }
    }

    let mut most_freq_minute = 0;
    let mut minute_frequency = 0;
    for (idx, &freq) in histogram.iter().enumerate() {
        if freq > minute_frequency {
            minute_frequency = freq;
            most_freq_minute = idx;
        }
    }

    target_guard_id as u64 * most_freq_minute as u64
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one(&input);
        assert_eq!(115167, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_two(&input);
        assert_eq!(32070, answer);
    }
}
