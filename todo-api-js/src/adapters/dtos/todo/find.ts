import { object, string, safeParse, transform } from "valibot";

import { Err, type Result } from "@domain/utils/result";
import { TodoEntity } from "@domain/entities/todo";
import { Id } from "@domain/value-objects/id";

import { type FindTodoInput, type FindTodoError } from "@application/dtos/todo/find";

import { ParseError, Request } from "../request";

export interface FindTodoPresenter<TView> {
    present(response: FindTodoResponse): TView;
}

export class FindTodoRequest extends Request<FindTodoInput> {
    parse(): Result<FindTodoInput, ParseError> {
        const req = safeParse(RequestSchema, this.src);
        if (!req.success) {
            return new Err(ParseError.fromValibot(req.issues));
        }

        return req.output.id
            .map((id) => ({ id }))
            .mapErr((err) => new ParseError({ key: "id", message: err.message }));
    }
}

export type FindTodoResponse = Result<TodoEntity, FindTodoError | ParseError>;

const RequestSchema = object({
    id: transform(string(), Id.parse),
});
