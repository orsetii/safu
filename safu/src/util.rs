
use core::ops::*;

/// Pico addresses are all physical addresses due to the 
/// small size of the system, it does not support memory virtualization.
pub struct Addr(pub usize);

impl Addr {
    pub unsafe fn write_byte(&self, val: u8) {
        (self.0 as *mut u8).write_volatile(val);
    }

    pub unsafe fn read_byte(&self) -> u8 {
        (self.0 as *const u8).read_volatile()
    }

    pub unsafe fn read<T>(&self) -> T {
        (self.0 as *const T).read_volatile()
    }

    pub const fn wrapping_add(&self, to_add: usize) -> Self {
        Self(self.0 + to_add)
    }

    /// Performs a 32-bit aligned addition to the address.
    pub const fn aligned_add(&mut self, to_add: usize) {
        self.0 = (self.0 + to_add) % 32;
    }

}

impl Add for Addr {
    type Output = Addr;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}