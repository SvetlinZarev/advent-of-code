use num_integer::lcm;

#[derive(Debug, Copy, Clone)]
pub struct Bus {
    interval: u64,
    offset: u64,
}

pub fn parse_input_data(input: &str) -> Vec<Bus> {
    let mut lines = input.lines();
    let _ignore = lines.next().unwrap();

    let mut departure_intervals = vec![];
    let mut intervals = lines.next().unwrap().split(',');

    let first = intervals.next().unwrap().parse().unwrap();
    departure_intervals.push(Bus {
        interval: first,
        offset: 0,
    });

    for (idx, interval) in intervals.into_iter().enumerate() {
        if interval == "x" {
            continue;
        }

        departure_intervals.push(Bus {
            interval: interval.parse().unwrap(),
            offset: idx as u64 + 1,
        });
    }

    departure_intervals
}

pub fn solve(busses: &[Bus]) -> u64 {
    let mut solution = busses[0].interval;
    let mut step = busses[0].interval;

    for bus in busses.iter().skip(1).copied() {
        while (solution + bus.offset) % bus.interval != 0 {
            solution += step;
        }

        // the solution must be increments of LCM of all encountered
        // intervals up to this point
        step = lcm(step, bus.interval);
    }

    solution
}
