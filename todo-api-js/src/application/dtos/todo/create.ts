import { type Result } from "@domain/utils/result";
import { type Description, type Title, type Status, type TodoEntity } from "@domain/entities/todo";
import { type CreateError } from "@application/repositories/todo";

export type CreateTodoInput = {
    title: Title;
    description?: Description;
    status: Status;
    todoAt?: Date;
};

export type CreateTodoOutput = Result<TodoEntity, CreateTodoError>;

export type CreateTodoError = CreateError;
