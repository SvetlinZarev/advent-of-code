use std::fmt::Debug;
use std::time::{Duration, Instant};

use crate::output::display_measurement;

pub fn measure<S, R, F>(day: usize, comment: S, f: F) -> (Duration, R)
    where
        S: AsRef<str>,
        R: Debug,
        F: FnOnce() -> R,
{
    let start = Instant::now();
    let answer = f();
    let duration = start.elapsed();

    let comment = comment.as_ref();

    if comment.contains("parsing") {
        display_measurement(day, comment, duration, &"N/A");
    } else {
        display_measurement(day, comment, duration, &answer);
    }

    (duration, answer)
}
