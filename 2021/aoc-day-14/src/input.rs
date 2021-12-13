#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rule {
    pub(crate) from: [u8; 2],
    pub(crate) to: [u8; 2],
}

impl Rule {
    pub fn new<S: AsRef<[u8]>>(from: S, insert: u8) -> Self {
        let from = from.as_ref();
        assert_eq!(2, from.len());

        Self {
            from: [from[0], from[1]],
            to: [from[0], insert],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct FastRule {
    pub(crate) key: [u8; 2],
    pub(crate) first: usize,
    pub(crate) second: usize,
}

impl FastRule {
    pub fn new(key: [u8; 2], first: usize, second: usize) -> Self {
        Self { key, first, second }
    }
}
