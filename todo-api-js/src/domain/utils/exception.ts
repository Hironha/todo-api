export abstract class Exception<TName extends string = string> {
    constructor(
        public readonly kind: TName,
        public readonly message: string,
        public readonly cause?: unknown
    ) {}
}

export class InternalError extends Exception<"InternalError"> {
    constructor(cause?: unknown) {
        super("InternalError", "Internal server error", cause);
    }
}
