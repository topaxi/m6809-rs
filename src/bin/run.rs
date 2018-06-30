extern crate m6809;
use m6809::mem::Mem;

fn main() {
    let ram = m6809::mem::Ram::new([0; 0x10000]);
    let mut eeprom = m6809::mem::EEPROM::new();
    let mut keyboard = m6809::keyboard::Keyboard::new();
    let mut pia1 = m6809::pia::PIA::new();
    let mut pia2 = m6809::pia::PIA::new();

    let mut cpu = m6809::cpu::Cpu::new(m6809::mem::MemMap::new(
            ram,
            &mut eeprom,
            &mut keyboard,
            &mut pia1,
            &mut pia2
    ));

    cpu.reset();
    let mut mem = [0; 0x10000];
    cpu.copy_into(&mut mem);
    mem.to_vec();
    //cpu.run_cycles(30);
}
