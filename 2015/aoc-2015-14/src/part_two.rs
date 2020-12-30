use std::cmp::min;

use crate::{Reindeer, SECONDS};

pub fn solve(reindeers: &[Reindeer]) -> u32 {
    let mut max_distance = 0;

    let mut distances = vec![0; reindeers.len()];
    let mut scores = vec![0; reindeers.len()];

    for second in 1..=SECONDS {
        for (idx, rd) in reindeers.iter().enumerate() {
            let time_per_cycle = rd.rest_duration + rd.flight_duration;
            let full_flights_duration = (second / time_per_cycle) * rd.flight_duration;
            let partial_flight_duration = min(second % time_per_cycle, rd.flight_duration);
            let flight_time = full_flights_duration + partial_flight_duration;
            let distance = flight_time * rd.speed;

            distances[idx] = distance;
            max_distance = max_distance.max(distance);
        }

        for (idx, &dist) in distances.iter().enumerate() {
            if dist == max_distance {
                scores[idx] += 1;
            }
        }
    }

    scores.iter().copied().max().unwrap()
}
