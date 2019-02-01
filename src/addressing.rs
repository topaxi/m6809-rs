use crate::cpu::Cpu;
use crate::mem::Mem;

trait AddressingMode<M: Mem> {
    fn read(&self, cpu: &mut Cpu<M>) -> u8;
    fn store(&self, cpu: &mut Cpu<M>, val: u8);
}

struct DirectAddressingMode;
impl<M: Mem> AddressingMode<M> for DirectAddressingMode {
    fn read(&self, cpu: &mut Cpu<M>) -> u8 {
        cpu.dp
    }
    fn store(&self, cpu: &mut Cpu<M>, val: u8) {
        cpu.dp = val
    }
}

struct AccumulatorAAddressingMode;
impl<M: Mem> AddressingMode<M> for AccumulatorAddressingMode {
    fn read(&self, cpu: &mut Cpu<M>) -> u8 {
        cpu.aa
    }
    fn store(&self, cpu: &mut Cpu<M>, val: u8) {
        cpu.aa = val
    }
}

struct AccumulatorBAddressingMode;
impl<M: Mem> AddressingMode<M> for AccumulatorAddressingMode {
    fn read(&self, cpu: &mut Cpu<M>) -> u8 {
        cpu.ab
    }
    fn store(&self, cpu: &mut Cpu<M>, val: u8) {
        cpu.ab = val
    }
}

struct ImmediateAddressingMode;
impl<M: Mem> AddressingMode<M> for ImmediateAddressingMode {
    fn read(&self, cpu: &mut Cpu<M>) -> u8 {
        cpu.fetch()
    }
    fn store(&self, _: &mut Cpu<M>, _: u8) {
        panic!("Unable to store to immediate")
    }
}

struct MemoryAddressingMode {
    val: u16,
}

impl Deref for MemoryAddressingMode {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.val
    }
}

impl<M: Mem> AddressingMode<M> for MemoryAddressingMode {
    fn read(&self, cpu: &mut Cpu<M>) -> u8 {
        cpu.read(**self)
    }
    fn store(&self, cpu: &mut Cpu<M>, val: u8) {
        cpu.store(**self, val)
    }
}
