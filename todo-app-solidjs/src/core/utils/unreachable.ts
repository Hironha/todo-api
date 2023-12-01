export function unreachable(v: never): never {
  throw new Error(`Reached unreachable case with value ${v}`);
}
