use std::hash::{BuildHasherDefault, Hasher};

pub type FxHashBuilder = BuildHasherDefault<rustc_hash::FxHasher>;
pub type FxHashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type FxHashSet<T> = rustc_hash::FxHashSet<T>;

pub type FnvHasherBuilder = BuildHasherDefault<FnvHasher>;
pub type FnvHashMap<K, V> = std::collections::HashMap<K, V, FnvHasherBuilder>;
pub type FnvHashSet<T> = std::collections::HashSet<T, FnvHasherBuilder>;

#[derive(Debug, Copy, Clone)]
pub struct FnvHasher(u64);

impl Default for FnvHasher {
    #[inline(always)]
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl Hasher for FnvHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;
        for byte in bytes {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        *self = FnvHasher(hash);
    }
}
