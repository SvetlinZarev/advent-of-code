pub fn parse_input_data(input: &str) -> (u32, Vec<u32>) {
    let mut lines = input.lines();
    let arrival_time = lines.next().unwrap().parse().unwrap();

    let mut departure_intervals = vec![];
    let intervals = lines.next().unwrap().split(',');
    for interval in intervals {
        if interval == "x" {
            continue;
        }

        departure_intervals.push(interval.parse().unwrap());
    }

    (arrival_time, departure_intervals)
}

pub fn solve(arrival: u32, intervals: &[u32]) -> u32 {
    let mut min_wait = u32::max_value();
    let mut solution = 0;

    for interval in intervals.iter().copied() {
        let time_to_wait = interval - arrival % interval;

        if time_to_wait < min_wait {
            min_wait = time_to_wait;
            solution = interval * time_to_wait;
        }
    }

    solution
}
