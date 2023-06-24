export type Left<L> = { value: L; state: 'left' }

export type Right<R> = { value: R; state: 'right' }

export type Either<L, R> = Left<L> | Right<R>

export function left<L, R = unknown>(val: L): Either<L, R> {
  return { value: val, state: 'left' }
}

export function right<R, L = unknown>(val: R): Either<L, R> {
  return { value: val, state: 'right' }
}

export function isRight<L = unknown, R = unknown>(either: Either<L, R>): either is Right<R> {
  return either.state === 'right'
}

export function isLeft<L = unknown, R = unknown>(either: Either<L, R>): either is Left<L> {
  return either.state === 'left'
}
