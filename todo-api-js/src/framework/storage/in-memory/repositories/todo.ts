import { Id } from "@domain/value-objects/id";
import { Err, Ok, type Result } from "@domain/utils/result";
import { InternalError } from "@domain/utils/exception";
import { TodoEntity } from "@domain/entities/todo";

import {
    IdNotFound,
    DuplicatedTitle,
    type UpdateQuery,
    type TodoRepository,
    type CreateError,
    type DeleteByIdError,
    type FindByIdError,
    type UpdateError,
    type ListPaginatedQuery,
    type PaginatedList,
    type ListError,
} from "@application/repositories/todo";

import { createFromEntity, mapToEntity, type TodoModel } from "../models/todo";

export class InMemoryTodoRepository implements TodoRepository {
    constructor(private todos: TodoModel[] = []) {}

    async create(entity: TodoEntity): Promise<Result<void, CreateError>> {
        const todoByTitle = this.todos.find((todo) => todo.title === entity.title.value);
        if (todoByTitle) {
            return new Err(new DuplicatedTitle(entity.title));
        }

        const todoById = this.todos.find((todo) => todo.id === entity.id.value);
        if (todoById) {
            return new Err(new InternalError());
        }

        this.todos.push(createFromEntity(entity));
        return new Ok(undefined);
    }

    async deleteById(todoId: Id): Promise<Result<void, DeleteByIdError>> {
        const index = this.todos.findIndex((todo) => todo.id === todoId.value);
        if (index === -1) {
            return new Err(new IdNotFound(todoId));
        }

        this.todos.splice(index, 1);
        return new Ok(undefined);
    }

    async findById(todoId: Id): Promise<Result<TodoEntity, FindByIdError>> {
        const todo = this.todos.find((todo) => todo.id === todoId.value);
        if (!todo) {
            return new Err(new IdNotFound(todoId));
        }

        return mapToEntity(todo);
    }

    async update(query: UpdateQuery): Promise<Result<void, UpdateError>> {
        const todoById = this.todos.find((todo) => todo.id === query.id.value);
        if (!todoById) {
            return new Err(new IdNotFound(query.id));
        }

        const todoByTitle = this.todos.find((todo) => todo.title === query.title.value);
        if (todoByTitle && todoByTitle.id !== todoById.id) {
            return new Err(new DuplicatedTitle(query.title));
        }

        todoById.title = query.title.value;
        todoById.description = query.description?.value;
        todoById.status = query.status.value;
        todoById.todoAt = query.todoAt?.toISOString();
        todoById.updatedAt = new Date().toISOString();

        return new Ok(undefined);
    }

    async listPaginated(query: ListPaginatedQuery): Promise<Result<PaginatedList, ListError>> {
        const offset = (query.page - 1) * query.limit;
        const entities: TodoEntity[] = [];
        for (let i = offset; i < this.todos.length; i++) {
            const entity = mapToEntity(this.todos[i]);
            if (entity.isErr()) {
                return entity;
            }
            entities.push(entity.value);
        }

        return new Ok({
            count: this.todos.length,
            data: entities,
            limit: query.limit,
            page: query.page,
        });
    }
}
