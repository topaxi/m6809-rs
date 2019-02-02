import Kit from './kit'
import { GUI } from './gui'

let frame = 0

function main(t) {
  let lastFrame = t
  const kit = new Kit(new Uint8Array(0x10000))
  kit.cpu.reset()
  kit.cpu.go(0x1000)

  const gui = new GUI(kit)
  document.body.appendChild(gui.render())

  const runloop = t => {
    let T = t - lastFrame
    let cycles = (gui.clock.hz * T * 1e-6) >>> 0

    if (cycles !== 0) {
      kit.cpu.run_cycles(cycles)
      lastFrame = t
      gui.update()
    }

    frame = window.requestAnimationFrame(runloop)
  }
  frame = window.requestAnimationFrame(runloop)
}

frame = window.requestAnimationFrame(main)
