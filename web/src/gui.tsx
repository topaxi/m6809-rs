import { Cpu, EEPROM, PIA } from '../m6809'
import { formatByte, formatWord } from './utils'
import LEDStrip from './gui/led-strip'
import MemView from './gui/mem-view'
import Keyboard from './gui/keyboard'
import Registers from './gui/registers';
import Kit from './kit'

export class GUI {
  readonly memView = new MemView()
  readonly ledstrip = new LEDStrip(this.kit.pia1)
  readonly keyboard = new Keyboard()
  readonly registers = new Registers()

  constructor(private readonly kit: Kit) {}

  render() {
    return (
      <>
        {this.memView.render(this.kit.cpu.dump())}
        {this.registers.render(this.kit.cpu.registers())}
        {this.keyboard.render(this.kit)}
        {this.ledstrip.render()}
      </>
    )
  }

  update() {
    this.memView.update(this.kit.cpu.dump())
    this.registers.update(this.kit.cpu.registers())
    this.ledstrip.update()
    this.keyboard.update(this.kit.eeprom)
  }
}
