export type ResultUnion<T, E> = { kind: "ok"; value: T } | { kind: "err"; value: E };

export interface BaseResult<T, E> {
  isOk(): this is Ok<T>;
  isErr(): this is Err<E>;
  map<U>(predicate: (value: T) => U): BaseResult<U, E>;
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

  isErr(): this is Err<never> {
    return false;
  }

  map<U>(predicate: (value: T) => U): BaseResult<U, never> {
    this._value = predicate(this._value) as any;
    return this as any;
  }

  ok(): T {
    return this.value;
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

  isOk(): this is Ok<never> {
    return false;
  }

  isErr(): this is Err<E> {
    return true;
  }

  map<U>(_predicate: (value: never) => U): BaseResult<U, E> {
    return this;
  }

  ok(): undefined {
    return undefined;
  }

  err(): E | undefined {
    return this.value;
  }
}
