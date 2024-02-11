import { Exception, type InternalError } from "@domain/utils/exception";
import { type Result } from "@domain/utils/result";
import { type Id } from "@domain/value-objects/id";
import { type Status, type Description, type Title, type TodoEntity } from "@domain/entities/todo";

export interface TodoRepository {
    findById(todoId: Id): Promise<Result<TodoEntity, FindByIdError>>;
    create(entity: TodoEntity): Promise<Result<void, CreateError>>;
    update(entity: UpdateQuery): Promise<Result<void, UpdateError>>;
    deleteById(todoId: Id): Promise<Result<void, DeleteByIdError>>;
    listPaginated(query: ListPaginatedQuery): Promise<Result<PaginatedList, ListError>>;
}

export type UpdateQuery = {
    id: Id;
    title: Title;
    description?: Description;
    status: Status;
    todoAt?: Date;
};

export type ListPaginatedQuery = { page: number; limit: number };

// TODO: maybe add a common type to this
export type PaginatedList = {
    count: number;
    page: number;
    limit: number;
    data: TodoEntity[];
};

export type FindByIdError = IdNotFound | InternalError;

export type CreateError = DuplicatedTitle | InternalError;

export type UpdateError = DuplicatedTitle | IdNotFound | InternalError;

export type DeleteByIdError = IdNotFound | InternalError;

export type ListError = InternalError;

export class DuplicatedTitle extends Exception<"DuplicatedTitle"> {
    constructor(title: Title) {
        super("DuplicatedTitle", `Todo with title ${title.value} already exists`);
    }
}

export class IdNotFound extends Exception<"IdNotFound"> {
    constructor(id: Id) {
        super("IdNotFound", `Todo with id ${id.value} could not be found`);
    }
}
