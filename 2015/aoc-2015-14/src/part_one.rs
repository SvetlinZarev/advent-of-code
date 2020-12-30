use std::cmp::min;

use crate::{Reindeer, SECONDS};

pub fn solve(reindeers: &[Reindeer]) -> u32 {
    let mut max_distance = 0;

    for rd in reindeers {
        let time_per_cycle = rd.rest_duration + rd.flight_duration;

        let full_flights_duration = (SECONDS / time_per_cycle) * rd.flight_duration;
        let partial_flight_duration = min(SECONDS % time_per_cycle, rd.flight_duration);

        let flight_time = full_flights_duration + partial_flight_duration;
        let distance = flight_time * rd.speed;

        max_distance = max_distance.max(distance);
    }

    max_distance
}
