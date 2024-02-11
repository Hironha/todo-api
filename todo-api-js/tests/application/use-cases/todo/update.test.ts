import { describe, test, expect } from "bun:test";

import { TodoEntity, Title, Status } from "@domain/entities/todo";
import { Id } from "@domain/value-objects/id";

import { UpdateTodoUseCase } from "@application/use-cases/todo/update";
import { DuplicatedTitle, IdNotFound } from "@application/repositories/todo";

import { InMemoryTodoRepository } from "../../repositories/todo.mock";

describe("Update todo use case", () => {
    test("Fail because todo with same title already exists", async () => {
        const firstTodo = TodoEntity.create({
            title: Title.parse("Todo 1").ok()!,
            status: Status.parse("done").ok()!,
        });
        const secondTodo = TodoEntity.create({
            title: Title.parse("Todo 2").ok()!,
            status: Status.parse("todo").ok()!,
        });
        const repository = new InMemoryTodoRepository([firstTodo, secondTodo]);

        const secondTodoProps = secondTodo.unpack();
        const interactor = new UpdateTodoUseCase(repository);
        const result = await interactor.exec({
            id: secondTodoProps.id,
            status: secondTodoProps.status,
            title: firstTodo.title,
        });

        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeInstanceOf(DuplicatedTitle);
        expect(result.err()).toBeInstanceOf(DuplicatedTitle);
    });

    test("Fail because todo could not be found", async () => {
        const repository = new InMemoryTodoRepository();

        const interactor = new UpdateTodoUseCase(repository);
        const result = await interactor.exec({
            id: Id.create(),
            title: Title.parse("title").ok()!,
            status: Status.parse("todo").ok()!,
        });

        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeInstanceOf(IdNotFound);
        expect(result.err()).toBeInstanceOf(IdNotFound);
    });

    test("Success on update todo", async () => {
        const todo = TodoEntity.create({
            title: Title.parse("Title").ok()!,
            status: Status.parse("done").ok()!,
        });
        const repository = new InMemoryTodoRepository([todo]);

        const interactor = new UpdateTodoUseCase(repository);
        const result = await interactor.exec({
            id: todo.id,
            title: Title.parse("Updated Title").ok()!,
            status: Status.parse("todo").ok()!,
        });

        expect(result.isOk()).toBeTrue();
        expect(result.value).toBeUndefined();
        expect(result.ok()).toBeUndefined();
    });
});
