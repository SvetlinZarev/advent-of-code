use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn default_input(day: usize) -> String {
    format!("puzzle-inputs/day-{:02}.txt", day)
}

pub fn default_test_input(day: usize) -> String {
    format!("../puzzle-inputs/day-{:02}.txt", day)
}

pub fn load_input<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref();
    let mut buffer = String::new();

    File::open(path)
        .map_err(|e| anyhow::Error::from(e).context(format!("Cannot open: {:?}", path)))
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();
    buffer
}
