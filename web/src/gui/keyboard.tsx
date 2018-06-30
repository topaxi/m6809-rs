import { EEPROM } from '../../m6809'
import Kit from '../kit'

export default class Keyboard {
  private readonly eepromEnableButton = (
    <button value="enable-eeprom">E</button>
  )

  render(kit: Kit) {
    return (
      <div class="hardware-keys" onclick={new HardwareKeyHandler(kit)}>
        <button value="reset">R</button>
        <button value="firq">F</button>
        {this.eepromEnableButton}
        <button value="disable-eeprom">D</button>
      </div>
    )
  }

  update(eeprom: EEPROM) {
    this.eepromEnableButton.classList.toggle('active', eeprom.writable)
  }
}

class HardwareKeyHandler {
  constructor(private readonly kit: Kit) {}

  handleEvent(e: Event) {
    if (e.target instanceof HTMLButtonElement) {
      switch (e.target.value) {
        case 'reset':
          this.kit.cpu.reset()
          break
        case 'firq':
          // TODO: Not implemented
          break
        case 'enable-eeprom':
          this.kit.eeprom.unlock()
          break
        case 'disable-eeprom':
          this.kit.eeprom.lock()
          break
      }
    }
  }
}
