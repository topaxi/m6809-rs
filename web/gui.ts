import { Cpu, EEPROM, Keyboard, PIA } from './m6809'
import Kit from './kit'

let ledstrip
let eepromEnableButton

export function createGUI(kit: Kit) {
  let hardwareKeys = document.querySelector('.hardware-keys')

  hardwareKeys.addEventListener('click', event => {
    if (event.target instanceof HTMLButtonElement) {
      switch (event.target.value) {
        case 'reset':
          kit.cpu.reset()
          break
        case 'firq':
          // TODO: Not implemented
          break
        case 'enable-eeprom':
          kit.eeprom.unlock()
          break
        case 'disable-eeprom':
          kit.eeprom.lock()
          break
      }
    }
  })

  ledstrip = initLEDStrip(kit.pia1)
  eepromEnableButton = document.querySelector('button[value=enable-eeprom]')
}

export function updateGUI({ eeprom, pia1 }: Kit) {
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
