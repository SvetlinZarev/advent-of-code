pub mod v1;
pub mod v2;

const DECRYPTION_KEY: i64 = 811_589_153;

// See https://stackoverflow.com/a/10184756/2588800
fn wrap(idx: i64, len: usize) -> usize {
    (((idx % len as i64) + len as i64) as usize) % len
}
