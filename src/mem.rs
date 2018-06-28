use keyboard::Keyboard;
use pia::PIA;
use wasm_bindgen::prelude::*;

pub trait Mem {
    fn read(&self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, val: u8);

    fn read_word(&self, addr: u16) -> u16 {
        ((self.read(addr) as u16) << 8) | (self.read(addr + 1) as u16)
    }

    fn store_word(&mut self, addr: u16, val: u16) {
        self.store(addr, (val & 0xFF) as u8);
        self.store(addr + 1, ((val >> 8) & 0xFF) as u8);
    }
}

pub struct Ram {
    val: [u8; 0x10000],
}

impl Ram {
    pub fn new(val: [u8; 0x10000]) -> Ram {
        Ram { val }
    }
}

impl Mem for Ram {
    fn read(&self, addr: u16) -> u8 {
        // Why?
        //self.val[addr as usize & 0x7FF]
        self.val[addr as usize & 0xFFFF]
    }

    fn store(&mut self, addr: u16, val: u8) {
        // Why?
        //self.val[addr as usize & 0x7FF] = val;
        self.val[addr as usize & 0xFFFF] = val;
    }
}

#[wasm_bindgen]
pub struct EEPROM {
    val:          [u8; 0x2000],
    #[wasm_bindgen(readonly)]
    pub writable: bool,
}

#[wasm_bindgen]
impl EEPROM {
    pub fn lock(&mut self) {
        self.writable = false;
    }

    pub fn unlock(&mut self) {
        self.writable = true;
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> EEPROM {
        EEPROM {
            val:      [0; 0x2000],
            writable: false,
        }
    }
}

impl Mem for EEPROM {
    fn read(&self, addr: u16) -> u8 {
        self.val[addr as usize & 0x1FFF]
    }

    fn store(&mut self, addr: u16, val: u8) {
        if self.writable {
            self.val[addr as usize & 0x1FFF] = val;
        }
    }
}

pub struct MemMap<'a> {
    pub ram:      Ram,
    pub eeprom:   &'a mut EEPROM,
    pub keyboard: &'a mut Keyboard,
    pub pia1:     &'a mut PIA,
    pub pia2:     &'a mut PIA,
}

impl<'a> MemMap<'a> {
    pub fn new(
        ram: Ram,
        eeprom: &'a mut EEPROM,
        keyboard: &'a mut Keyboard,
        pia1: &'a mut PIA,
        pia2: &'a mut PIA,
    ) -> MemMap<'a> {
        MemMap {
            ram,
            eeprom,
            keyboard,
            pia1,
            pia2,
        }
    }
}

impl<'a> Mem for MemMap<'a> {
    fn read(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            // EEPROM
            self.eeprom.read(addr)
        } else if addr < 0x2600 {
            // RAM
            self.ram.read(addr)
        } else if addr < 0x2800 {
            // System-Stack
            self.ram.read(addr)
        } else if addr < 0x2900 {
            // PIA1
            self.pia1.read(addr - 0x2800)
        } else if addr < 0x2A00 {
            // PIA2
            self.pia1.read(addr - 0x2900)
        } else if addr < 0x2C00 {
            // Ext.BUS
            0
        } else if addr < 0x2D00 {
            // Empty
            0
        } else if addr < 0x2E00 {
            // Display
            0
        } else if addr < 0x2F00 {
            // Keyboard
            self.keyboard.read(addr - 0x2E00)
        } else if addr < 0x3000 {
            // Input and IRQ-FF
            0
        } else if addr < 0x4000 {
            // Ram (Subroutines)
            self.ram.read(addr)
        } else if addr < 0x6000 {
            // USER/RAM
            self.ram.read(addr)
        } else if addr < 0x8000 {
            // USER/RAM
            self.ram.read(addr)
        } else if addr < 0xA000 {
            // RAM (Download)
            self.ram.read(addr)
        } else if addr < 0xC000 {
            // Ext.BUS
            0
        } else
        /* if addr <= 0xFFFF */
        {
            // EPROM
            self.ram.read(addr)
        }
    }

    fn store(&mut self, addr: u16, val: u8) {
        if addr <= 0x1FFF {
            // EEPROM
            self.eeprom.store(addr, val)
        } else if addr < 0x2600 {
            // RAM
            self.ram.store(addr, val)
        } else if addr < 0x2800 {
            // System-Stack
            self.ram.store(addr, val)
        } else if addr < 0x2900 {
            // PIA1
            self.pia1.store(addr - 0x2800, val)
        } else if addr < 0x2A00 {
            // PIA2
            self.pia1.store(addr - 0x2900, val)
        } else if addr < 0x2C00 {
            // Ext.BUS
        } else if addr < 0x2D00 {
            // Empty
        } else if addr < 0x2E00 {
            // Display
        } else if addr < 0x2F00 {
            // Keyboard
            self.keyboard.store(addr - 0x2E00, val)
        } else if addr < 0x3000 {
            // Input and IRQ-FF
        } else if addr < 0x4000 {
            // Ram (Subroutines)
            self.ram.store(addr, val)
        } else if addr < 0x6000 {
            // USER/RAM
            self.ram.store(addr, val)
        } else if addr < 0x8000 {
            // USER/RAM
            self.ram.store(addr, val)
        } else if addr < 0xA000 {
            // RAM (Download)
            self.ram.store(addr, val)
        } else if addr < 0xC000 {
            // EPROM
            self.ram.store(addr, val)
        }
    }
}
