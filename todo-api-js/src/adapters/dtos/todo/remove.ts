import { object, string, safeParse, transform } from "valibot";

import { Err, type Result } from "@domain/utils/result";
import { type Exception } from "@domain/utils/exception";
import { Id } from "@domain/value-objects/id";

import { ParseError, Request } from "../request";
import { type RemoveTodoInput, type RemoveTodoError } from "@application/dtos/todo/remove";

export interface RemoveTodoPresenter<TView> {
    present(result: Result<void, RemoveTodoError | Exception>): TView;
}

export class RemoveTodoRequest extends Request<RemoveTodoInput> {
    parse(): Result<RemoveTodoInput, ParseError> {
        const req = safeParse(RequestSchema, this.src);
        if (!req.success) {
            return new Err(ParseError.fromValibot(req.issues));
        }

        return req.output.id
            .map((id) => ({ id }))
            .mapErr((err) => new ParseError({ key: "id", message: err.message }));
    }
}

export type RemoveTodoResponse = Result<void, RemoveTodoError | ParseError>;

const RequestSchema = object({
    id: transform(string(), Id.parse),
});
