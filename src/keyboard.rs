use mem::Mem;

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
    keys: [u8; 0xFF],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { keys: [0; 0xFF] }
    }
}

impl Mem for Keyboard {
    fn read(&self, addr: u16) -> u8 {
        self.keys[addr as usize & 0xFF]
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.keys[addr as usize & 0xFF] = val;
    }
}
