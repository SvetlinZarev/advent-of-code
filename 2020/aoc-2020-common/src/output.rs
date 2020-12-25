use std::fmt::Debug;
use std::time::{Duration, Instant};

pub fn print_result<R: Debug>(day: u32, part: u32, comment: &str, result: R, duration: Duration) {
    let duration = format!("{:.3?}", duration);
    println!(
        "{:>9} | Day {:02} | Part {:02} | Comment: {:9} | Result: {:?}",
        duration, day, part, comment, result
    );
}

pub fn measure_solution<R: Debug, F: FnOnce() -> R>(
    day: u32,
    part: u32,
    comment: &str,
    solution: F,
) -> R {
    let start = Instant::now();
    let result = solution();
    let duration = start.elapsed();

    print_result(day, part, comment, &result, duration);

    result
}
