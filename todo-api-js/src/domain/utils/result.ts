export type Result<T, E> = Ok<T> | Err<E>;

export interface BaseResult<T, E> {
    isOk(): this is Ok<T>;
    isErr(): this is Err<E>;
    ok(): T | undefined;
    err(): E | undefined;
    map<U>(predicate: (value: T) => U): Result<U, E>;
    mapErr<U>(predicate: (err: E) => U): Result<T, U>;
    expect(msg: string): T;
}

export class Ok<T> implements BaseResult<T, never> {
    readonly value: T;

    constructor(value: T) {
        this.value = value;
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

    map<U>(predicate: (value: T) => U): Ok<U> {
        return new Ok(predicate(this.value));
    }

    mapErr(): Ok<T> {
        return this;
    }

    expect(): T {
        return this.value;
    }
}

export class Err<E> implements BaseResult<never, E> {
    readonly value: E;

    constructor(value: E) {
        this.value = value;
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

    map(): Err<E> {
        return this;
    }

    mapErr<U>(predicate: (err: E) => U): Err<U> {
        return new Err(predicate(this.value));
    }

    expect(msg: string): never {
        throw new Error(msg);
    }
}
