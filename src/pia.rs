use mem::Mem;

pub struct PIA {
    val: [u8; 0xFF]
}

impl PIA {
    pub fn new() -> PIA {
        PIA { val: [0; 0xFF] }
    }
}

impl Mem for PIA {
    fn read(&self, addr: u16) -> u8 {
        self.val[addr as usize & 0xFF]
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.val[addr as usize & 0xFF] = val;
    }
}
