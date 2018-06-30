import { formatByte, formatWord } from '../utils'

const Cell = ({ addr, ...rest }, ...children) => (
  <li class="mem-cell" data-addr={formatWord(addr)} {...rest}>
    {children}
  </li>
)

export default class MemView {
  handleEventOptions = { passive: true, capture: true }

  private readonly limit = 128
  private readonly rowHeight = 24
  private memView: Text[] = []
  private cells: HTMLLIElement[] = []
  private readonly view = <ol id="memView" />
  private readonly container = (
    <div id="mem" onscroll={this}>
      {this.view}
    </div>
  )
  private prevMem: Uint8Array = new Uint8Array()
  private lastRenderY = 0
  private toClean = new Set<HTMLLIElement>()
  private cleanupId = null
  private renderId = null

  render(mem: Uint8Array) {
    this.container.appendChild(
      <div style={`height: ${mem.length * this.rowHeight}px`} />,
    )

    this.memView = Array.from(mem, byte => <text>0x{formatByte(byte)}</text>)
    this.cells = this.memView.map((text, i) => (
      <Cell addr={formatWord(i)} style={`top: ${i * this.rowHeight}px`}>
        {text}
      </Cell>
    ))

    this.view.appendChild(<>{this.cells.slice(0, this.limit)}</>)
    this.prevMem = mem
    return this.container
  }

  handleEvent(e: Event & { target: HTMLDivElement }) {
    if (this.renderId !== null) window.cancelAnimationFrame(this.renderId)
    this.renderId = window.requestAnimationFrame(this.onScroll)
  }

  private onScroll = () => {
    this.renderId = null
    let scrollTop = this.container.scrollTop | 0 // Triggers reflow
    if (
      Math.abs(scrollTop - this.lastRenderY) >
      (this.limit * this.rowHeight) / 4
    ) {
      let first = ((scrollTop / this.rowHeight) | 0) - this.limit / 4
      this.renderList(Math.max(0, first))
      this.scheduleNodeRemoval()
      this.lastRenderY = scrollTop
    }
  }

  private scheduleNodeRemoval() {
    if (this.cleanupId !== null) window.cancelIdleCallback(this.cleanupId)
    this.cleanupId = window.requestIdleCallback(() => {
      this.cleanupId = null
      this.toClean.forEach(el => el.remove())
      this.toClean.clear()
    })
  }

  renderList(index) {
    let finalItem = Math.min(this.cells.length, index + this.limit)

    for (let i = 0; i < this.view.children.length; i++) {
      this.toClean.add(this.view.children[i] as any)
    }

    let fragment = document.createDocumentFragment()
    for (let i = index; i < finalItem; i++) {
      this.toClean.delete(this.cells[i])
      fragment.appendChild(this.cells[i])
    }

    this.view.appendChild(fragment)
  }

  update(mem: Uint8Array) {
    for (let i = 0; i < mem.length; i++) {
      if (mem[i] !== this.prevMem[i]) {
        this.memView[i].nodeValue = `0x${formatByte(mem[i])}`
      }
    }

    this.prevMem = mem
  }
}
