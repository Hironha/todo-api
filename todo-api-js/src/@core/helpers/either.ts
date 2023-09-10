export type Left<L> = { value: L }
export type Right<R> = { value: R }
export type Either<L, R> = Left<L> | Right<R>

export class Mapping<L, R> {
  constructor(private either: Either<L, R>) {}

  /** Transforms `R` into `T` using the callback `fn` */
  mapRight<T>(fn: (value: R) => T): Mapping<L, T> {
    if (isLeft(this.either)) {
      return this as unknown as Mapping<L, T>
    }
    return new Mapping(right(fn(this.either.value)))
  }

  /** Transforms `L` into `T` using the callback `fn` */
  mapLeft<T>(fn: (value: L) => T): Mapping<T, R> {
    if (isRight(this.either)) {
      return this as unknown as Mapping<T, R>
    }
    return new Mapping(left(fn(this.either.value)))
  }

  /** Unwraps to `Either<L, R>` */
  unwrap(): Either<L, R> {
    return this.either
  }
}

export function right<R, L = unknown>(val: R): Either<L, R> {
  const either: Either<L, R> = { value: val }
  Object.defineProperty(either, 'state', { configurable: false, value: 'r' })
  return either
}

export function left<L, R = unknown>(val: L): Either<L, R> {
  const either: Either<L, R> = { value: val }
  Object.defineProperty(either, 'state', { configurable: false, value: 'l' })
  return either
}

export function isRight<L = unknown, R = unknown>(either: Either<L, R>): either is Right<R> {
  const state = Object.getOwnPropertyDescriptor(either, 'state')
  return state?.value === 'r'
}

export function isLeft<L = unknown, R = unknown>(either: Either<L, R>): either is Left<L> {
  const state = Object.getOwnPropertyDescriptor(either, 'state')
  return state?.value === 'l'
}

/** Transforms a `Either<L, R>` to `R | null` */
export function asRight<L = unknown, R = unknown>(either: Either<L, R>): R | null {
  return isRight(either) ? either.value : null
}

/** Transforms a `Either<L, R>` to `L | null` */
export function asLeft<L = unknown, R = unknown>(either: Either<L, R>): L | null {
  return isLeft(either) ? either.value : null
}

/** Creates a chainable mapper of `Either<L, R>`, allowing to transform `L` and `R` without removing the `Either` constraint */
export function map<L = unknown, R = unknown>(either: Either<L, R>): Mapping<L, R> {
  return new Mapping(either)
}
