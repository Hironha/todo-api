import { type Result, Ok } from "./result";

export class ArrayUtils {
  static tryMap<T, U, E>(arr: T[], predicate: (item: T) => Result<U, E>): Result<U[], E> {
    const collection: U[] = [];

    for (const item of arr) {
      const mapped = predicate(item);
      if (mapped.isOk()) {
        collection.push(mapped.value);
      } else {
        return mapped
      }
    }

    return new Ok(collection);
  }
}
