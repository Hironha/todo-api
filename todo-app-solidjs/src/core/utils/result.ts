export type ResultUnion<T, E> = { kind: "ok"; value: T } | { kind: "err"; value: E };

export class Result<T, E> {
  private variant: Ok<T> | Err<E>;

  private constructor(variant: Ok<T> | Err<E>) {
    this.variant = variant;
  }

  static fromUnion<S, U>(union: ResultUnion<S, U>): Result<S, U> {
    if (union.kind === "ok") {
      return new Result<S, U>(new Ok(union.value));
    } else {
      return new Result<S, U>(new Err(union.value));
    }
  }

  static ok<S, U>(value: S): Result<S, U> {
    return new Result<S, U>(new Ok(value));
  }

  static err<S, U>(value: U): Result<S, U> {
    return new Result<S, U>(new Err(value));
  }

  get value(): T | E {
    return this.variant.value;
  }

  isOk(): this is Ok<T> {
    return this.variant instanceof Ok;
  }

  isErr(): this is Err<E> {
    return this.variant instanceof Err;
  }

  ok(): Ok<T> | undefined {
    return this.variant instanceof Ok ? this.variant : undefined;
  }

  err(): Err<E> | undefined {
    return this.variant instanceof Err ? this.variant : undefined;
  }

  map<U>(predicate: (value: T) => U): Result<U, E> {
    if (this.isOk()) {
      return Result.ok(predicate(this.value));
    } else {
      return this as unknown as Result<U, E>;
    }
  }

  union(): ResultUnion<T, E> {
    if (this.isOk()) {
      return { kind: "ok", value: this.value };
    } else {
      return { kind: "err", value: this.value as E };
    }
  }
}

export class Ok<T> {
  private v: T;

  constructor(value: T) {
    this.v = value;
  }

  get value(): T {
    return this.v;
  }
}

export class Err<E> {
  private v: E;

  constructor(value: E) {
    this.v = value;
  }

  get value(): E {
    return this.v;
  }
}
