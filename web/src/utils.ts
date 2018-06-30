export function toHex(num: number): string {
  return num.toString(16).toUpperCase()
}

export function formatByte(byte: number): string {
  return toHex(byte).padStart(2, '0')
}

export function formatWord(word: number): string {
  return toHex(word).padStart(4, '0')
}
