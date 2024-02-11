import { object, string, optional, coerce, date, safeParse, transform } from "valibot";

import { Err, Ok, type Result } from "@domain/utils/result";
import { Description, Status, Title } from "@domain/entities/todo";

import { type UpdateTodoInput, type UpdateTodoError } from "@application/dtos/todo/update";

import { ParseError, Request } from "../request";
import { Id } from "@domain/value-objects/id";

export interface UpdateTodoPresenter<TView> {
    present(response: UpdateTodoResponse): TView;
}

export class UpdateTodoRequest extends Request<UpdateTodoInput> {
    parse(): Result<UpdateTodoInput, ParseError> {
        const req = safeParse(RequestSchema, this.src);
        if (!req.success) {
            return new Err(ParseError.fromValibot(req.issues));
        }

        const { output: schema } = req;
        if (schema.id.isErr()) {
            return schema.id.mapErr((err) => new ParseError({ key: "id", message: err.message }));
        }

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
            id: schema.id.value,
            title: schema.title?.value,
            description: schema.description?.value,
            status: schema.status?.value,
            todoAt: schema.todoAt,
        });
    }
}

export type UpdateTodoResponse = Result<void, UpdateTodoError | ParseError>;

const RequestSchema = object({
    id: transform(string(), Id.parse),
    title: transform(string(), Title.parse),
    description: optional(transform(string(), Description.parse)),
    status: transform(string(), Status.parse),
    todoAt: optional(coerce(date(), coerceDate)),
});

function coerceDate(value: unknown): Date {
    return new Date(value ? (value as any) : {});
}
