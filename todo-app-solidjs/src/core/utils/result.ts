export interface BaseResult<T, E> {
  isOk(): this is Ok<T>;
  isErr(): this is Err<E>;
  ok(): T | undefined;
  err(): E | undefined;
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
    return this._value;
  }

  err(): undefined {
    return undefined;
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
    return this._value;
  }
}
