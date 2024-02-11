import { describe, test, expect } from "bun:test";

import { TodoEntity, Title, Status } from "@domain/entities/todo";
import { Id } from "@domain/value-objects/id";

import { RemoveTodoUseCase } from "@application/use-cases/todo/remove";
import { IdNotFound } from "@application/repositories/todo";

import { InMemoryTodoRepository } from "../../repositories/todo.mock";

describe("Update todo use case", () => {
    test("Fail because todo could not be found", async () => {
        const repository = new InMemoryTodoRepository();

        const interactor = new RemoveTodoUseCase(repository);
        const result = await interactor.exec({ id: Id.create() });

        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeInstanceOf(IdNotFound);
        expect(result.err()).toBeInstanceOf(IdNotFound);
    });

    test("Success removing todo", async () => {
        const todo = TodoEntity.create({
            title: Title.parse("Title").ok()!,
            status: Status.parse("done").ok()!,
        });
        const repository = new InMemoryTodoRepository([todo]);

        const interactor = new RemoveTodoUseCase(repository);
        const result = await interactor.exec({ id: todo.id });

        expect(result.isOk()).toBeTrue();
        expect(result.value).toBeUndefined();
        expect(result.ok()).toBeUndefined();
    });
});
