import { describe, test, expect } from "bun:test";

import { TodoEntity, Title, Status } from "@domain/entities/todo";

import { CreateTodoUseCase } from "@application/use-cases/todo/create";
import { DuplicatedTitle } from "@application/repositories/todo";

import { InMemoryTodoRepository } from "../../repositories/todo.mock";

describe("Create todo use case", () => {
    test("Fail because todo with same title already exists", async () => {
        const title = Title.parse("Title").ok()!;
        const status = Status.parse("done").ok()!;
        const todo = TodoEntity.create({ title, status });
        const repository = new InMemoryTodoRepository([todo]);

        const interactor = new CreateTodoUseCase(repository);
        const result = await interactor.exec({ title, status });

        expect(result.isErr()).toBeTrue();
        expect(result.value).toBeInstanceOf(DuplicatedTitle);
        expect(result.err()).toBeInstanceOf(DuplicatedTitle);
    });

    test("Success creating todo entity", async () => {
        const repository = new InMemoryTodoRepository();
        const interactor = new CreateTodoUseCase(repository);
        const result = await interactor.exec({
            title: Title.parse("Title").ok()!,
            status: Status.parse("done").ok()!,
        });

        expect(result.isOk()).toBeTrue();
        expect(result.value).toBeInstanceOf(TodoEntity);
        expect(result.ok()).toBeInstanceOf(TodoEntity);
    });
});
