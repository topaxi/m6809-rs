extern crate m6809;

fn main() {
    let mut cpu = m6809::cpu::Cpu::new(m6809::mem::MemMap::new(
        m6809::mem::Ram::new([0; 0x10000]),
        m6809::keyboard::Keyboard::new(),
        m6809::pia::PIA::new(),
        m6809::pia::PIA::new(),
    ));

    cpu.reset();
    cpu.run_cycles(30);
}
