import Kit from './kit'
import { GUI } from './gui'

const CPU_HZ = 1.0e6
let frame = 0

//setTimeout(() => window.cancelAnimationFrame(frame), 1000)

function main(t) {
  let lastFrame = t
  const kit = new Kit(new Uint8Array(0x10000))
  kit.cpu.reset()
  kit.cpu.go(0x1000)

  const gui = new GUI(kit)
  document.body.appendChild(gui.render())

  const runloop = t => {
    let T = t - lastFrame
    let cycles = Math.min(30, (CPU_HZ * T * 1e-6) >>> 0)

    kit.cpu.run_cycles(cycles)
    lastFrame = t
    gui.update()
    frame = window.requestAnimationFrame(runloop)
  }
  frame = window.requestAnimationFrame(runloop)
}

frame = window.requestAnimationFrame(main)
