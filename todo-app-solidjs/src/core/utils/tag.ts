declare const __tag: unique symbol;

export type Tagged<TName extends string> = { [__tag]: TName };

/** Create a tagged type, i.e. opaque data type */
export type Tag<T, TName extends string> = T & Tagged<TName>;
