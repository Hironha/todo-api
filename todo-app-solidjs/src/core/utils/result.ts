export type Ok<T = void> = T extends void
  ? { status: "ok" }
  : T extends undefined
  ? { status: "ok"; data?: T }
  : { status: "ok"; data: T };

export type Err<E = void> = E extends void
  ? { status: "err" }
  : E extends undefined
  ? { status: "err"; error?: E }
  : { status: "err"; error: E };
