import { Elysia } from "elysia";

import { CreateTodoUseCase } from "@application/use-cases/todo/create";
import { FindTodoUseCase } from "@application/use-cases/todo/find";
import { ListTodosUseCase } from "@application/use-cases/todo/list";
import { RemoveTodoUseCase } from "@application/use-cases/todo/remove";
import { UpdateTodoUseCase } from "@application/use-cases/todo/update";
import { ListTodosRequest } from "@adapters/dtos/todo/list";
import {
    CreateTodoJsonPresenter,
    FindTodoJsonPresenter,
    ListTodosJsonPresenter,
    RemoveTodoJsonPresenter,
    UpdateTodoJsonPresenter,
} from "@adapters/presenters/json/todo/presenter";
import { CreateTodoRequest } from "@adapters/dtos/todo/create";
import { FindTodoRequest } from "@adapters/dtos/todo/find";
import { RemoveTodoRequest } from "@adapters/dtos/todo/remove";
import { UpdateTodoRequest } from "@adapters/dtos/todo/update";
import { ListTodosController } from "@adapters/controllers/todo/list";
import { FindTodoController } from "@adapters/controllers/todo/find";
import { CreateTodoController } from "@adapters/controllers/todo/create";
import { RemoveTodoController } from "@adapters/controllers/todo/remove";
import { UpdateTodoController } from "@adapters/controllers/todo/update";
import { InMemoryTodoRepository } from "@framework/storage/in-memory/repositories/todo";

export function createRouter(app: Elysia<"/todos">) {
    return app
        .decorate("repository", new InMemoryTodoRepository())
        .get("", async (ctx) => {
            const presenter = new ListTodosJsonPresenter();
            const interactor = new ListTodosUseCase(ctx.repository);
            const controller = new ListTodosController(interactor, presenter);
            const request = new ListTodosRequest({
                page: Number(ctx.query.page),
                limit: Number(ctx.query.limit),
            });
            console.debug(request);

            const response = await controller.run(request);
            if (response.isErr()) {
                console.error(response.value);
                ctx.set.status = response.value.status;
                return response.value.error;
            }

            ctx.set.status = "OK";
            return response.value;
        })
        .post(
            "",
            async (ctx) => {
                const presenter = new CreateTodoJsonPresenter();
                const interactor = new CreateTodoUseCase(ctx.repository);
                const controller = new CreateTodoController(interactor, presenter);
                const request = new CreateTodoRequest(ctx.body ?? {});
                console.debug(request);

                const response = await controller.run(request);
                if (response.isErr()) {
                    console.error(response.value);
                    ctx.set.status = response.value.status;
                    return response.value.error;
                }

                ctx.set.status = "Created";
                return response.value;
            },
            { type: "json" }
        )
        .get("/:todoId", async (ctx) => {
            const presenter = new FindTodoJsonPresenter();
            const interactor = new FindTodoUseCase(ctx.repository);
            const controller = new FindTodoController(interactor, presenter);
            const request = new FindTodoRequest({ id: ctx.params.todoId });
            console.debug(request);

            const response = await controller.run(request);
            if (response.isErr()) {
                console.error(response.value);
                ctx.set.status = response.value.status;
                return response.value.error;
            }

            ctx.set.status = "OK";
            return response.value;
        })
        .put(
            "/:todoId",
            async (ctx) => {
                const presenter = new UpdateTodoJsonPresenter();
                const interactor = new UpdateTodoUseCase(ctx.repository);
                const controller = new UpdateTodoController(interactor, presenter);
                const body: Record<PropertyKey, any> = ctx.body || {};
                const request = new UpdateTodoRequest({
                    id: ctx.params.todoId,
                    title: body.title,
                    description: body.description,
                    status: body.status,
                    todoAt: body.todoAt,
                });

                const response = await controller.run(request);
                if (response.isErr()) {
                    console.error(response.value);
                    ctx.set.status = response.value.status;
                    return response.value.error;
                }

                ctx.set.status = 200;
                return response.value;
            },
            { type: "json" }
        )
        .delete("/:todoId", async (ctx) => {
            const presenter = new RemoveTodoJsonPresenter();
            const interactor = new RemoveTodoUseCase(ctx.repository);
            const controller = new RemoveTodoController(interactor, presenter);
            const request = new RemoveTodoRequest({ id: ctx.params.todoId });
            console.debug(request);

            const response = await controller.run(request);
            if (response.isErr()) {
                console.error(response.value);
                ctx.set.status = response.value.status;
                return response.value.error;
            }

            ctx.set.status = "OK";
            return response.value;
        });
}
