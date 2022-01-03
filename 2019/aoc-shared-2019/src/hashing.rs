use std::hash::{BuildHasher, Hasher};
use std::marker::PhantomData;

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

const FX_ROTATE: u32 = 5;
const FX_SEED64: u64 = 0x517cc1b727220a95;

#[derive(Debug, Copy, Clone)]
pub struct FxHasher {
    hash: u64,
}

impl Hasher for FxHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut bytes = bytes;

        // TODO: use unwrap_unchecked() when it is stabilized
        while bytes.len() >= 8 {
            let chunk: [u8; 8] = bytes[..8].try_into().unwrap();
            let num = u64::from_ne_bytes(chunk);
            self.write_u64(num);
            bytes = &bytes[8..];
        }

        if bytes.len() >= 4 {
            let chunk: [u8; 4] = bytes[..4].try_into().unwrap();
            let num = u32::from_ne_bytes(chunk) as u64;
            self.write_u64(num);
            bytes = &bytes[4..];
        }

        for &byte in bytes {
            self.write_u64(byte as u64);
        }
    }

    #[inline]
    fn write_u8(&mut self, num: u8) {
        self.write_u64(num as u64);
    }

    #[inline]
    fn write_u16(&mut self, num: u16) {
        self.write_u64(num as u64);
    }

    #[inline]
    fn write_u32(&mut self, num: u32) {
        self.write_u64(num as u64);
    }

    #[inline]
    fn write_u64(&mut self, num: u64) {
        self.hash = (self.hash.rotate_left(FX_ROTATE) ^ num).wrapping_mul(FX_SEED64);
    }

    #[inline]
    fn write_u128(&mut self, num: u128) {
        self.write_u64((num >> 64) as u64);
        self.write_u64(num as u64);
    }

    #[inline]
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn write_usize(&mut self, num: usize) {
        self.write_u64(num as u64);
    }
}
