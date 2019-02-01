use crate::mem::Mem;

// Function Keys
// B: Breakpoint
// S: Single Step
// G: Go
// X: Exit
// F: Select Function
// R: Select Register
// M: Display Memory
// *: Enter

// Number Keys
// 0-F

pub struct Keyboard {
    pub keys: [u8; 0x100],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { keys: [0; 0x100] }
    }
}

impl Mem for Keyboard {
    fn read(&self, addr: u16) -> u8 {
        self.keys[addr as usize & 0xFF]
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.keys[addr as usize & 0xFF] = val;
    }

    fn copy_into(&self, slice: &mut [u8]) {
        slice.copy_from_slice(&self.keys);
    }
}
