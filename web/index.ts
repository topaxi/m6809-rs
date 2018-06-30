import { formatByte, formatWord } from './src/utils'
import './src/create-element'
import 'core-js/modules/es7.array.flat-map'

Object.assign(window, {
  logOpcode(pc, op) {
    console.log('pc: 0x%s, op: 0x%s', formatWord(pc), formatByte(op))
  },
  saveEEPROM(eeprom: Uint8Array) {
    localStorage.setItem(
      'EEPROM',
      Array.from(eeprom, byte => formatByte(byte)).join(''),
    )
  },
  loadEEPROM(): Uint8Array {
    let rom = new Uint8Array(0x2000)
    let eeprom = localStorage.getItem('EEPROM')

    if (eeprom) {
      eeprom
        .match(/.{2}/g)
        .forEach((byteStr, i) => (rom[i] = parseInt(byteStr, 16) & 0xff))
    }

    return rom
  },
})

import('./src/main').catch(console.error)
