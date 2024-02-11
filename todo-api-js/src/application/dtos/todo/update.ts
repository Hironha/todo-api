import { type Result } from "@domain/utils/result";
import { type Id } from "@domain/value-objects/id";
import { type Description, type Title, type Status } from "@domain/entities/todo";
import { type UpdateError, type FindByIdError } from "@application/repositories/todo";

export type UpdateTodoInput = {
    id: Id;
    title: Title;
    description?: Description;
    status: Status;
    todoAt?: Date;
};

export type UpdateTodoOutput = Result<void, UpdateTodoError>;

export type UpdateTodoError = UpdateError | FindByIdError;
