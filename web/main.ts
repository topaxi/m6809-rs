import Kit from './kit'
import { createGUI, updateGUI } from './gui'

;(window as any).logOpcode = function(pc, op) {
  console.log(
    'pc: 0x%s, op: 0x%s',
    pc.toString(16).padStart(4, '0'),
    op.toString(16).padStart(2, '0'),
  )
}

const CPU_HZ = 1.5e6
let frame = 0

//setTimeout(() => window.cancelAnimationFrame(frame), 1000)

function main(t) {
  let lastFrame = t
  const kit = new Kit(new Uint8Array(0x10000))
  createGUI(kit);
  kit.cpu.reset()
  kit.cpu.go(0x1000)

  const runloop = t => {
    let T = t - lastFrame
    let cycles = Math.min(30, (CPU_HZ * T * 1e-6) >>> 0)

    kit.cpu.run_cycles(cycles)
    lastFrame = t
    updateGUI(kit)
    frame = window.requestAnimationFrame(runloop)
  }
  frame = window.requestAnimationFrame(runloop)
}

frame = window.requestAnimationFrame(main)
