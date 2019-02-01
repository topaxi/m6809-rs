use crate::mem::Mem;

pub struct PIA {
    pub val: [u8; 0x100],
}

impl PIA {
    pub fn new() -> PIA {
        PIA { val: [0; 0x100] }
    }
}

impl Mem for PIA {
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        self.val[addr as usize & 0xFF]
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.val[addr as usize & 0xFF] = val;
    }

    fn copy_into(&self, slice: &mut [u8]) {
        slice.copy_from_slice(&self.val);
    }
}
