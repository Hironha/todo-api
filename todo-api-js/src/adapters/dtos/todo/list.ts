import { object, number, integer, minValue, maxValue, safeParse } from "valibot";

import { Err, Ok, type Result } from "@domain/utils/result";

import {
    type ListTodosInput,
    type ListTodosError,
    type TodoList,
} from "@application/dtos/todo/list";

import { ParseError, Request } from "../request";

export interface ListTodosPresenter<TView> {
    present(response: ListTodosResponse): TView;
}

export class ListTodosRequest extends Request<ListTodosInput> {
    parse(): Result<ListTodosInput, ParseError> {
        const req = safeParse(RequestSchema, this.src);
        if (!req.success) {
            return new Err(ParseError.fromValibot(req.issues));
        }

        return new Ok(req.output);
    }
}

export type ListTodosResponse = Result<TodoList, ListTodosError | ParseError>;

const RequestSchema = object({
    page: number([integer(), minValue(1)]),
    limit: number([integer(), minValue(10), maxValue(100)]),
});
