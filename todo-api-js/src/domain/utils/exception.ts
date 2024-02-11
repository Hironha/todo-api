export abstract class Exception<TName extends string = string> extends Error {
    readonly name: TName;

    constructor(name: TName, msg: string, cause?: Error) {
        super(msg, { cause });
        this.name = name;
    }
}

export class InternalError extends Exception<"InternalError"> {
    constructor(cause?: Error) {
        super("InternalError", "Internal server error", cause);
    }

    static fromUnknown(err: unknown): InternalError {
        return new InternalError(err instanceof Error ? err : undefined);
    }
}
