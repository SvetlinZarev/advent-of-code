use std::fmt::Debug;
use std::time::Duration;

pub fn display_measurement<A>(day: u32, comment: &str, duration: Duration, answer: &A)
where
    A: Debug,
{
    let duration = format!("{:.3?}", duration);
    println!(
        "Day {:02} | {:16} | {:>9} | {:?}",
        day, comment, duration, answer
    );
}
