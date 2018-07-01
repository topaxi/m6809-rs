import { PIA } from '../../m6809'
import { toHex, range } from '../utils'

class SwitchHandler {
  constructor(private readonly pia: PIA) {}

  handleEvent(e: Event & { target: HTMLInputElement }) {
    let byte = this.pia.read(0x02)
    let value = Number(e.target.value)

    this.pia.store(0x02, e.target.checked ? byte | value : byte & ~value)
  }
}

const LED = attrs => <input {...attrs} type="checkbox" disabled />
const Switch = attrs => <input {...attrs} type="checkbox" />

export default class LEDStrip {
  private length = 8
  private readonly leds = range(0, this.length, i => (
    <LED name={i} value={this.valueFor(i)} />
  ))
  private readonly switchHandler = new SwitchHandler(this.pia)
  private readonly switches = range(0, this.length, i => (
    <Switch
      name={toHex(i + this.length)}
      value={this.valueFor(i)}
      onclick={this.switchHandler}
    />
  ))

  constructor(private readonly pia: PIA) {}

  render() {
    return (
      <div class="ledstrip">
        <div class="leds">{this.leds}</div>
        <div class="switches">{this.switches}</div>
      </div>
    )
  }

  update() {
    let byte = this.pia.read(0x00)

    for (let i = 0; i < this.leds.length; i++) {
      this.leds[i].checked = (byte & Number(this.leds[i].value)) !== 0
    }
  }

  private valueFor(i: number) {
    return `0b${(1 << i).toString(2).padStart(this.length, '0')}`
  }
}
