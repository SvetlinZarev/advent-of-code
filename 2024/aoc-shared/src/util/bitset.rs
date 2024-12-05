#[derive(Clone, Hash, Eq, PartialEq)]
pub struct BitSet {
    bits: Vec<usize>,
}

impl BitSet {
    #[inline(always)]
    pub fn new(bits: usize) -> Self {
        let mut size = bits / usize::BITS as usize;
        size += ((bits % usize::BITS as usize) != 0) as usize;

        Self {
            bits: vec![0usize; size],
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    #[inline(always)]
    pub fn is_set(&self, bit: usize) -> bool {
        let (idx, pos) = self.keys(bit);
        (self.bits[idx] & (1 << pos)) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, bit: usize) {
        let (idx, pos) = self.keys(bit);
        self.bits[idx] |= 1 << pos;
    }

    #[inline(always)]
    pub fn unset(&mut self, bit: usize) {
        let (idx, pos) = self.keys(bit);
        self.bits[idx] &= !(1 << pos);
    }

    #[inline(always)]
    pub fn mark(&mut self, bit: usize) -> bool {
        let (idx, pos) = self.keys(bit);

        if self.bits[idx] & (1 << pos) != 0 {
            return false;
        }

        self.bits[idx] |= 1 << pos;
        true
    }

    #[inline(always)]
    pub fn count_ones(&self) -> usize {
        self.bits
            .iter()
            .fold(0, |acc, &x| acc + x.count_ones() as usize)
    }

    #[inline(always)]
    fn keys(&self, bit: usize) -> (usize, usize) {
        let idx = bit / usize::BITS as usize;
        let pos = bit % usize::BITS as usize;

        (idx, pos)
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct SmallBitSet<const N: usize> {
    bits: [u64; N],
}

impl<const N: usize> SmallBitSet<N> {
    #[inline(always)]
    pub fn new() -> Self {
        Self { bits: [0u64; N] }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    pub fn bits(&self) -> usize {
        self.bits.len() * (u64::BITS as usize)
    }

    #[inline(always)]
    pub fn is_set(&self, bit: usize) -> bool {
        let (idx, pos) = self.keys(bit);
        (self.bits[idx] & (1 << pos)) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, bit: usize) {
        let (idx, pos) = self.keys(bit);
        self.bits[idx] |= 1 << pos;
    }

    #[inline(always)]
    pub fn unset(&mut self, bit: usize) {
        let (idx, pos) = self.keys(bit);
        self.bits[idx] &= !(1 << pos);
    }

    #[inline(always)]
    pub fn mark(&mut self, bit: usize) -> bool {
        let (idx, pos) = self.keys(bit);

        if self.bits[idx] & (1 << pos) != 0 {
            return false;
        }

        self.bits[idx] |= 1 << pos;
        true
    }

    #[inline(always)]
    fn keys(&self, bit: usize) -> (usize, usize) {
        let idx = bit / u64::BITS as usize;
        let pos = bit % u64::BITS as usize;

        (idx, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bitset() {
        let bitset = BitSet::new(usize::BITS as usize - 1);
        assert_eq!(1, bitset.bits.len());

        let bitset = BitSet::new(usize::BITS as usize);
        assert_eq!(1, bitset.bits.len());

        let bitset = BitSet::new(usize::BITS as usize + 1);
        assert_eq!(2, bitset.bits.len());
    }

    #[test]
    fn test_bitset_no_set_bits() {
        let bitset = BitSet::new(usize::BITS as usize);
        for bit in 0..usize::BITS as usize {
            assert_eq!(false, bitset.is_set(bit));
        }
    }

    #[test]
    fn test_bitset_set_one_bit() {
        let mut bitset = BitSet::new(usize::BITS as usize);

        for to_set in 0..usize::BITS as usize {
            bitset.clear();
            bitset.set(to_set);

            for to_get in 0..usize::BITS as usize {
                assert_eq!(to_get == to_set, bitset.is_set(to_get));
            }
        }
    }

    #[test]
    fn test_bitset_unset_one_bit() {
        let mut bitset = BitSet::new(usize::BITS as usize);

        for to_unset in 0..usize::BITS as usize {
            for bit in 0..usize::BITS as usize {
                bitset.set(bit);
            }

            bitset.unset(to_unset);

            for to_get in 0..usize::BITS as usize {
                assert_eq!(to_get != to_unset, bitset.is_set(to_get));
            }
        }
    }

    #[test]
    fn test_create_small_bitset() {
        let bitset = SmallBitSet::<1>::new();
        assert_eq!(1, bitset.bits.len());

        let bitset = SmallBitSet::<4>::new();
        assert_eq!(4, bitset.bits.len());
    }

    #[test]
    fn test_small_bitset_no_set_bits() {
        let bitset = SmallBitSet::<1>::new();
        for bit in 0..bitset.bits() {
            assert_eq!(false, bitset.is_set(bit));
        }
    }

    #[test]
    fn test_small_bitset_set_one_bit() {
        let mut bitset = SmallBitSet::<3>::new();

        for to_set in 0..bitset.bits() {
            bitset.clear();
            bitset.set(to_set);

            for to_get in 0..bitset.bits() {
                assert_eq!(to_get == to_set, bitset.is_set(to_get));
            }
        }
    }

    #[test]
    fn test_small_bitset_unset_one_bit() {
        let mut bitset = SmallBitSet::<3>::new();

        for to_unset in 0..bitset.bits() {
            for bit in 0..bitset.bits() {
                bitset.set(bit);
            }

            bitset.unset(to_unset);

            for to_get in 0..bitset.bits() {
                assert_eq!(to_get != to_unset, bitset.is_set(to_get));
            }
        }
    }
}
