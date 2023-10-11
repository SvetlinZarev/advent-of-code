use std::hash::{BuildHasher, Hasher};
use std::marker::PhantomData;

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

#[derive(Copy, Clone)]
pub struct HashBuilder<H> {
    _phantom: PhantomData<H>,
}

impl<H: Hasher + Default> BuildHasher for HashBuilder<H> {
    type Hasher = H;

    fn build_hasher(&self) -> Self::Hasher {
        H::default()
    }
}

impl<H: Hasher + Default> Default for HashBuilder<H> {
    fn default() -> Self {
        HashBuilder {
            _phantom: PhantomData,
        }
    }
}
