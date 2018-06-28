import { Cpu, EEPROM, Keyboard, PIA } from './m6809'

export default class Kit {
  readonly eeprom = new EEPROM()
  readonly keyboard = new Keyboard()
  readonly pia1 = new PIA()
  readonly pia2 = new PIA()
  readonly cpu: Cpu

  constructor(memory: Uint8Array) {
    this.cpu = new Cpu(
      memory,
      this.eeprom,
      this.keyboard,
      this.pia1,
      this.pia2,
    )
    this.eeprom.unlock()
    loadSubroutines(this.cpu)
    load(this.cpu)
    this.eeprom.lock()
  }
}

function loadSubroutines(cpu: Cpu) {
  // RTS
  cpu.store(0x3044, 0x39)
}

function load(cpu: Cpu) {
  cpu.store(0x1000, 0xbd)
  cpu.store(0x1001, 0x30)
  cpu.store(0x1002, 0x44)

  cpu.store(0x1003, 0xb6)
  cpu.store(0x1004, 0x28)
  cpu.store(0x1005, 0x02)

  cpu.store(0x1006, 0x88) // 3a
  //cpu.store(0x1006, 0x84) // 3b
  cpu.store(0x1007, 0xf0)

  cpu.store(0x1008, 0xb7)
  cpu.store(0x1009, 0x28)
  cpu.store(0x100a, 0x00)

  cpu.store(0x100b, 0x7e)
  cpu.store(0x100c, 0x10)
  cpu.store(0x100d, 0x03)
}

function load3c(cpu: Cpu) {
  cpu.store(0x1000, 0xbd)
  cpu.store(0x1001, 0x30)
  cpu.store(0x1002, 0x44)

  cpu.store(0x1003, 0xb6)
  cpu.store(0x1004, 0x28)
  cpu.store(0x1005, 0x02)

  cpu.store(0x1006, 0x88)
  cpu.store(0x1007, 0xf0)

  cpu.store(0x1008, 0x84)
  cpu.store(0x1009, 0x7f)

  cpu.store(0x100a, 0x8a)
  cpu.store(0x100b, 0x01)

  cpu.store(0x100c, 0xb7)
  cpu.store(0x100d, 0x28)
  cpu.store(0x100e, 0x00)

  cpu.store(0x100f, 0x7e)
  cpu.store(0x1010, 0x10)
  cpu.store(0x1011, 0x03)
}
