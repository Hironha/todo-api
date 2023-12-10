/** Useful to safely assert a unreachable situation */
export function unreachable(v: never): never {
  console.error(`Reached unreachable case with value ${v}`)
  return v
}
