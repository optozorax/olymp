#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BitVec64(pub u64);

impl BitVec64 {
    #[inline(always)]
    pub fn get_bit(&self, pos: u8) -> bool {
        debug_assert!(pos < 64);
        ((1 << pos) & self.0) >> pos == 1
    }

    #[inline(always)]
    pub fn set_bit(&mut self, pos: u8, value: bool) {
        if value {
            self.set_true(pos);
        } else {
            self.set_false(pos);
        }
    }

    #[inline(always)]
    pub fn set_true(&mut self, pos: u8) {
        debug_assert!(pos < 64);
        self.0 |= 1 << pos;
    }

    #[inline(always)]
    pub fn set_false(&mut self, pos: u8) {
        debug_assert!(pos < 64);
        self.0 &= !(1 << pos);
    }
}