import { type Id } from "@domain/value-objects/id";
import { Err, Ok, type Result } from "@domain/utils/result";
import { TodoEntity } from "@domain/entities/todo";

import {
    IdNotFound,
    DuplicatedTitle,
    type TodoRepository,
    type CreateError,
    type DeleteByIdError,
    type FindByIdError,
    type UpdateError,
    type ListPaginatedQuery,
    type PaginatedList,
    type ListError,
    type UpdateQuery,
} from "@application/repositories/todo";

export class InMemoryTodoRepository implements TodoRepository {
    constructor(private todos: TodoEntity[] = []) {}

    async create(entity: TodoEntity): Promise<Result<void, CreateError>> {
        if (this.todos.find((todo) => todo.title.eq(entity.title))) {
            return new Err(new DuplicatedTitle(entity.title));
        }
        this.todos.push(entity);
        return new Ok(undefined);
    }

    async deleteById(todoId: Id): Promise<Result<void, DeleteByIdError>> {
        const index = this.todos.findIndex((todo) => todo.id.eq(todoId));
        if (index === -1) {
            return new Err(new IdNotFound(todoId));
        }

        this.todos.splice(index, 1);
        return new Ok(undefined);
    }

    async findById(todoId: Id): Promise<Result<TodoEntity, FindByIdError>> {
        const todo = this.todos.find((todo) => todo.id.eq(todoId));
        if (!todo) {
            return new Err(new IdNotFound(todoId));
        }
        return new Ok(todo);
    }

    async update(query: UpdateQuery): Promise<Result<void, UpdateError>> {
        const todoByIdIdx = this.todos.findIndex((todo) => todo.id.eq(query.id));
        if (todoByIdIdx === -1) {
            return new Err(new IdNotFound(query.id));
        }

        const todoByTitle = this.todos.find((todo) => todo.title.eq(query.title));
        if (todoByTitle) {
            return new Err(new DuplicatedTitle(query.title));
        }

        const updated = TodoEntity.from({
            id: query.id,
            title: query.title,
            description: query.description,
            status: query.status,
            todoAt: query.todoAt,
            createdAt: this.todos[todoByIdIdx].unpack().createdAt,
            updatedAt: new Date(),
        });

        this.todos[todoByIdIdx] = updated;

        return new Ok(undefined);
    }

    async listPaginated(query: ListPaginatedQuery): Promise<Result<PaginatedList, ListError>> {
        return new Ok({
            count: this.todos.length,
            data: this.todos.slice((query.page - 1) * query.limit),
            limit: query.limit,
            page: query.page,
        });
    }
}
