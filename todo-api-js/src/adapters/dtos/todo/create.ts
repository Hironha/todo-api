import { object, string, optional, date, coerce, safeParse, transform } from "valibot";

import { Err, Ok, type Result } from "@domain/utils/result";
import { Description, Status, Title, TodoEntity } from "@domain/entities/todo";

import { type CreateTodoError, type CreateTodoInput } from "@application/dtos/todo/create";

import { ParseError, Request } from "../request";

export interface CreateTodoPresenter<TView> {
    present(response: CreateTodoResponse): TView;
}

export class CreateTodoRequest extends Request<CreateTodoInput> {
    parse(): Result<CreateTodoInput, ParseError> {
        const req = safeParse(RequestSchema, this.src);
        if (!req.success) {
            return new Err(ParseError.fromValibot(req.issues));
        }

        const { output: schema } = req;
        if (schema.title.isErr()) {
            return schema.title.mapErr((message) => new ParseError({ key: "title", message }));
        }

        if (schema.description && schema.description.isErr()) {
            return schema.description.mapErr(
                (message) => new ParseError({ key: "description", message })
            );
        }

        if (schema.status.isErr()) {
            return schema.status.mapErr((message) => new ParseError({ key: "status", message }));
        }

        return new Ok({
            title: schema.title.value,
            description: schema.description?.value,
            status: schema.status.value,
            todoAt: schema.todoAt,
        });
    }
}

export type CreateTodoResponse = Result<TodoEntity, CreateTodoError | ParseError>;

const RequestSchema = object({
    title: transform(string(), Title.parse),
    description: optional(transform(string(), Description.parse)),
    status: transform(string(), Status.parse),
    todoAt: optional(coerce(date(), coerceDate)),
});

function coerceDate(value: unknown): Date {
    return new Date(value ? (value as any) : {});
}
