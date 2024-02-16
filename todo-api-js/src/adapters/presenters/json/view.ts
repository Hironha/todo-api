export type Json =
    | string
    | number
    | boolean
    | null
    | void
    | undefined
    | Json[]
    | { [key: string]: Json };

export type JsonView<T extends Json> = T;
