import { Cpu, Registers } from '../../m6809'
import { formatByte, formatWord } from '../utils'

export default class {
  readonly ix = <text />
  readonly iy = <text />
  readonly su = <text />
  readonly ss = <text />

  readonly pc = <text />

  readonly aa = <text />
  readonly ab = <text />

  readonly dp = <text />
  readonly cc = <text />

  private prevRegisters: Registers = {} as any

  private renderRegister(name: string) {
    return (
      <>
        <dt>{name}</dt>
        <dd>{this[name]}</dd>
      </>
    )
  }

  render(regs: Registers) {
    this.update(regs)

    return (
      <dl class="registers">
        {this.renderRegister('ix')}
        {this.renderRegister('iy')}

        {this.renderRegister('su')}
        {this.renderRegister('ss')}

        {this.renderRegister('pc')}

        {this.renderRegister('aa')}
        {this.renderRegister('ab')}

        {this.renderRegister('dp')}
        {this.renderRegister('cc')}
      </dl>
    )
  }

  private cloneRegisters(registers: Registers): Registers {
    const { ix, iy, su, ss, pc, aa, ab, dp, cc } = registers
    return { ix, iy, su, ss, pc, aa, ab, dp, cc } as Registers
  }

  private updateWordRegister(regs: Registers, name: string) {
    if (regs[name] !== this.prevRegisters[name]) {
      this[name].nodeValue = formatWord(regs[name])
    }
  }

  private updateByteRegister(regs: Registers, name: string) {
    if (regs[name] !== this.prevRegisters[name]) {
      this[name].nodeValue = formatByte(regs[name])
    }
  }

  update(regs: Registers) {
    let registers = this.cloneRegisters(regs)

    this.updateWordRegister(registers, 'ix')
    this.updateWordRegister(registers, 'iy')

    this.updateWordRegister(registers, 'su')
    this.updateWordRegister(registers, 'ss')

    this.updateWordRegister(registers, 'pc')

    this.updateByteRegister(registers, 'aa')
    this.updateByteRegister(registers, 'ab')

    this.updateByteRegister(registers, 'dp')
    this.updateByteRegister(registers, 'cc')

    this.prevRegisters = registers
  }
}
