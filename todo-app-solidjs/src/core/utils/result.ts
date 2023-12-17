export type ResultUnion<T, E> = { kind: "ok"; value: T } | { kind: "err"; value: E };

export interface BaseResult<T, E> {
  isOk(): this is Ok<T>;
  isErr(): this is Err<E>;
  ok(): T | undefined;
  err(): E | undefined;
  map<U>(predicate: (value: T) => U): Result<U, E>;
  mapErr<U>(predicate: (err: E) => U): Result<T, U>;
}

export type Result<T, E> = Ok<T> | Err<E>;

export class Ok<T> implements BaseResult<T, never> {
  private _value: T;

  constructor(value: T) {
    this._value = value;
  }

  get value(): T {
    return this._value;
  }

  isOk(): this is Ok<T> {
    return true;
  }

  isErr(): this is never {
    return false;
  }

  ok(): T {
    return this.value;
  }

  err(): undefined {
    return undefined;
  }

  map<U>(predicate: (value: T) => U): Result<U, never> {
    return new Ok(predicate(this.value));
  }

  mapErr(): Result<T, never> {
    return this;
  }
}

export class Err<E> implements BaseResult<never, E> {
  private _value: E;

  constructor(value: E) {
    this._value = value;
  }

  get value(): E {
    return this._value;
  }

  isOk(): this is never {
    return false;
  }

  isErr(): this is Err<E> {
    return true;
  }

  ok(): undefined {
    return undefined;
  }

  err(): E | undefined {
    return this.value;
  }

  map(): Result<never, E> {
    return this;
  }

  mapErr<U>(predicate: (err: E) => U): Result<never, U> {
    return new Err(predicate(this.value));
  }
}
