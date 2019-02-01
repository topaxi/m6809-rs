use crate::cpu;
use crate::keyboard;
use crate::mem;
use crate::mem::Mem;
use crate::mem::MemMap;
use crate::mem::Ram;
use crate::pia;
use crate::registers::Registers;
use std::ops::Deref;
use std::ops::DerefMut;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = saveEEPROM)]
    fn save_eeprom(eeprom: Vec<u8>);

    #[wasm_bindgen(js_namespace = window, js_name = loadEEPROM)]
    fn load_eeprom() -> Vec<u8>;
}

macro_rules! to_array {
    ($bytes:expr, $x:expr) => {{
        let mut array = [0; $x];
        let bytes = &$bytes[..array.len()]; // panics if not enough data
        array.clone_from_slice(bytes);
        array
    }};
}

#[wasm_bindgen]
pub struct Cpu {
    cpu: cpu::Cpu<MemMap<'static>>,
}

#[wasm_bindgen]
impl Cpu {
    pub fn dump(&self) -> Vec<u8> {
        let mut mem = [0; 0x10000];
        self.cpu.copy_into(&mut mem);
        mem.to_vec()
    }

    pub fn registers(&self) -> Registers {
        self.cpu.registers()
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.cpu.read(addr)
    }

    pub fn store(&mut self, addr: u16, byte: u8) {
        self.cpu.store(addr, byte)
    }

    pub fn go(&mut self, addr: u16) {
        self.cpu.go(addr);
    }

    pub fn step(&mut self) -> u32 {
        self.cpu.step() as u32
    }

    pub fn run_cycles(&mut self, cycles: u32) {
        self.cpu.run_cycles(cycles as u64);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        mem: &[u8],
        eeprom: &mut EEPROM,
        keyboard: &mut Keyboard,
        pia1: &mut PIA,
        pia2: &mut PIA,
    ) -> Cpu {
        unsafe {
            Cpu {
                cpu: cpu::Cpu::new(MemMap::new(
                    Ram::new(to_array!(mem, 0x10000)),
                    // These are safe to leak as they are created in
                    // javascript and have to be freed manually anyways.
                    Box::leak(Box::from_raw(eeprom)) as &mut mem::EEPROM,
                    Box::leak(Box::from_raw(keyboard)) as &mut keyboard::Keyboard,
                    Box::leak(Box::from_raw(pia1)) as &mut pia::PIA,
                    Box::leak(Box::from_raw(pia2)) as &mut pia::PIA,
                )),
            }
        }
    }
}

#[wasm_bindgen]
pub struct EEPROM {
    eeprom: mem::EEPROM,
    #[wasm_bindgen(readonly)]
    pub writable: bool,
}

#[wasm_bindgen]
impl EEPROM {
    pub fn read(&self, addr: u16) -> u8 {
        self.eeprom.read(addr)
    }

    pub fn store(&mut self, addr: u16, byte: u8) {
        self.eeprom.store(addr, byte)
    }

    pub fn lock(&mut self) {
        self.writable = false;
        self.eeprom.lock();
        save_eeprom(self.eeprom.val.to_vec())
    }

    pub fn unlock(&mut self) {
        self.writable = true;
        self.eeprom.unlock()
    }

    pub fn dump(&self) -> Vec<u8> {
        self.eeprom.val.to_vec()
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> EEPROM {
        EEPROM {
            eeprom:   mem::EEPROM::new(to_array!(load_eeprom(), 0x2000)),
            writable: false,
        }
    }
}

impl Deref for EEPROM {
    type Target = mem::EEPROM;

    fn deref(&self) -> &mem::EEPROM {
        &self.eeprom
    }
}

impl DerefMut for EEPROM {
    fn deref_mut(&mut self) -> &mut mem::EEPROM {
        &mut self.eeprom
    }
}

#[wasm_bindgen]
pub struct Keyboard {
    keyboard: keyboard::Keyboard,
}

#[wasm_bindgen]
impl Keyboard {
    pub fn read(&self, addr: u16) -> u8 {
        self.keyboard.read(addr)
    }

    pub fn store(&mut self, addr: u16, byte: u8) {
        self.keyboard.store(addr, byte)
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Keyboard {
        Keyboard {
            keyboard: keyboard::Keyboard::new(),
        }
    }
}

impl Deref for Keyboard {
    type Target = keyboard::Keyboard;

    fn deref(&self) -> &keyboard::Keyboard {
        &self.keyboard
    }
}

impl DerefMut for Keyboard {
    fn deref_mut(&mut self) -> &mut keyboard::Keyboard {
        &mut self.keyboard
    }
}

#[wasm_bindgen]
pub struct PIA {
    pia: pia::PIA,
}

#[wasm_bindgen]
impl PIA {
    pub fn read(&self, addr: u16) -> u8 {
        self.pia.read(addr)
    }

    pub fn store(&mut self, addr: u16, byte: u8) {
        self.pia.store(addr, byte)
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> PIA {
        PIA {
            pia: pia::PIA::new(),
        }
    }
}

impl Deref for PIA {
    type Target = pia::PIA;

    fn deref(&self) -> &pia::PIA {
        &self.pia
    }
}

impl DerefMut for PIA {
    fn deref_mut(&mut self) -> &mut pia::PIA {
        &mut self.pia
    }
}
