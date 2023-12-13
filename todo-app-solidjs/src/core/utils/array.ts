import { type Result, Ok } from "./result";

export function tryMapArr<T, U, E>(arr: T[], predicate: (item: T) => Result<U, E>): Result<U[], E> {
  const collection: U[] = [];

  for (const item of arr) {
    const mapped = predicate(item);
    if (mapped.isErr()) {
      return mapped;
    }

    collection.push(mapped.value);
  }

  return new Ok(collection);
}
