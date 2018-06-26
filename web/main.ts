import { Cpu, EEPROM, Keyboard, PIA } from './m6809'
;(window as any).logOpcode = function(pc, op) {
  console.log(
    'pc: 0x%s, op: 0x%s',
    pc.toString(16).padStart(4, '0'),
    op.toString(16).padStart(2, '0'),
  )
}

import('./m6809').then(({ Cpu, EEPROM, Keyboard, PIA }) => {
  const CPU_HZ = 1.5e6
  let ledstrip
  let eepromEnableButton
  let frame = 0

  //setTimeout(() => window.cancelAnimationFrame(frame), 1000)

  function main(t) {
    let lastFrame = t
    const components = createCpu()
    createGUI(components)
    const { cpu } = components
    cpu.reset()
    cpu.go(0x1000)

    const runloop = t => {
      let T = t - lastFrame
      let cycles = Math.min(30, (CPU_HZ * T * 1e-6) >>> 0)

      cpu.run_cycles(cycles)
      lastFrame = t
      updateGUI(components)
      frame = window.requestAnimationFrame(runloop)
    }
    frame = window.requestAnimationFrame(runloop)
  }

  frame = window.requestAnimationFrame(main)

  function createCpu() {
    const memory = new Uint8Array(0x10000)
    loadSubroutines(memory)
    load(memory)
    const eeprom = new EEPROM()
    const keyboard = new Keyboard()
    const pia1 = new PIA()
    const pia2 = new PIA()
    const cpu = new Cpu(memory, eeprom, keyboard, pia1, pia2)

    return { cpu, eeprom, memory, keyboard, pia1, pia2 }
  }

  function createGUI(components: ReturnType<typeof createCpu>) {
    let hardwareKeys = document.querySelector('.hardware-keys')

    hardwareKeys.addEventListener('click', event => {
      if (event.target instanceof HTMLButtonElement) {
        switch (event.target.value) {
          case 'reset':
            components.cpu.reset()
            break
          case 'firq':
            // TODO: Not implemented
            break
          case 'enable-eeprom':
            components.eeprom.writable = true
            break
          case 'disable-eeprom':
            components.eeprom.writable = false
            break
        }
      }
    })

    ledstrip = initLEDStrip(components.pia1)
    eepromEnableButton = document.querySelector('button[value=enable-eeprom]')
  }

  function updateGUI({ eeprom, pia1 }: ReturnType<typeof createCpu>) {
    eepromEnableButton.classList.toggle('active', eeprom.writable)
    updateLEDStrip(pia1)
  }

  function initLEDStrip(pia: PIA) {
    let container = document.getElementById('ledstrip') as HTMLDivElement

    let ledstrip = Array.from(
      { length: 0x10 },
      (_v, k) =>
        container.querySelector<HTMLInputElement>(
          `[name="${k.toString(16).toUpperCase()}"]`,
        )!,
    )

    ledstrip.slice(0x8).forEach((v, k) => {
      v.checked = false
      v.addEventListener('click', function() {
        let byte = pia.read(0x02)
        let value = Number(this.value)

        pia.store(0x02, this.checked ? byte | value : byte & ~value)
      })
    })

    return ledstrip
  }

  function updateLEDStrip(pia: PIA) {
    let byte = pia.read(0x00)
    //console.log('in', pia.read(0x02), 'out', byte)

    ledstrip[0].checked = (byte & 0b0000_0001) !== 0
    ledstrip[1].checked = (byte & 0b0000_0010) !== 0
    ledstrip[2].checked = (byte & 0b0000_0100) !== 0
    ledstrip[3].checked = (byte & 0b0000_1000) !== 0
    ledstrip[4].checked = (byte & 0b0001_0000) !== 0
    ledstrip[5].checked = (byte & 0b0010_0000) !== 0
    ledstrip[6].checked = (byte & 0b0100_0000) !== 0
    ledstrip[7].checked = (byte & 0b1000_0000) !== 0
  }

  function load(mem: Uint8Array) {
    mem[0x1000] = 0xbd
    mem[0x1001] = 0x30
    mem[0x1002] = 0x44

    mem[0x1003] = 0xb6
    mem[0x1004] = 0x28
    mem[0x1005] = 0x02

    mem[0x1006] = 0x88 // 3a
    //mem[0x1006] = 0x84 // 3b
    mem[0x1007] = 0xf0

    mem[0x1008] = 0xb7
    mem[0x1009] = 0x28
    mem[0x100a] = 0x00

    mem[0x100b] = 0x7e
    mem[0x100c] = 0x10
    mem[0x100d] = 0x03
  }

  function load3c(mem: Uint8Array) {
    mem[0x1000] = 0xbd
    mem[0x1001] = 0x30
    mem[0x1002] = 0x44

    mem[0x1003] = 0xb6
    mem[0x1004] = 0x28
    mem[0x1005] = 0x02

    mem[0x1006] = 0x88
    mem[0x1007] = 0xf0

    mem[0x1008] = 0x84
    mem[0x1009] = 0x7f

    mem[0x100a] = 0x8a
    mem[0x100b] = 0x01

    mem[0x100c] = 0xb7
    mem[0x100d] = 0x28
    mem[0x100e] = 0x00

    mem[0x100f] = 0x7e
    mem[0x1010] = 0x10
    mem[0x1011] = 0x03
  }

  function loadSubroutines(mem: Uint8Array) {
    // RTS
    mem[0x3044] = 0x39
  }
})
