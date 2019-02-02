export const DEFAULT_CPU_HZ = 1e6

export default class Clock {
  private _hz = DEFAULT_CPU_HZ
  private readonly speed = (
    <input
      type="range"
      onchange={this}
      step={1e3}
      min={1e3}
      max={5e6}
      value={this._hz}
    />
  )
  private readonly label = <text>{this._hz}</text>

  get hz(): number {
    return this._hz
  }

  render() {
    return (
      <div>
        {this.speed}
        {this.label}
      </div>
    )
  }

  update() {
    let str = this.formatHertz(this._hz)

    if (this.label.nodeValue !== str) {
      this.label.nodeValue = str
    }
  }

  private handleEvent(e: Event) {
    this._hz = this.speed.value
  }

  private formatHertz(n: number): string {
    if (n >= 1e9) return `${n / 1e9}GHz`
    if (n >= 1e6) return `${n / 1e6}MHz`
    if (n >= 1e3) return `${n / 1e3}kHz`
    return `${n}Hz`
  }
}
