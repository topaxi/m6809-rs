import { Cpu, EEPROM, PIA } from '../m6809'
import { formatByte, formatWord } from './utils'
import Clock from './gui/clock'
import LEDStrip from './gui/led-strip'
import MemView from './gui/mem-view'
import Keyboard from './gui/keyboard'
import Registers from './gui/registers'
import Kit from './kit'

export class GUI {
  readonly clock = new Clock()
  readonly memView = new MemView()
  readonly ledstrip = new LEDStrip(this.kit.pia1)
  readonly keyboard = new Keyboard()
  readonly registers = new Registers()

  constructor(private readonly kit: Kit) {}

  render() {
    let registers = this.kit.cpu.registers()

    return (
      <>
        {this.memView.render(this.kit.cpu.dump(), registers)}
        <div>
          {this.clock.render()}
          {this.registers.render(registers)}
        </div>
        {this.keyboard.render(this.kit)}
        {this.ledstrip.render()}
      </>
    )
  }

  update() {
    let registers = this.kit.cpu.registers()

    this.memView.update(this.kit.cpu.dump(), registers)
    this.clock.update()
    this.registers.update(registers)
    this.ledstrip.update()
    this.keyboard.update(this.kit.eeprom)
  }
}
