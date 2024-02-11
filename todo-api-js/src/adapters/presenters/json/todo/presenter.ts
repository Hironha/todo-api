import { type Result } from "@domain/utils/result";

import { type CreateTodoResponse, type CreateTodoPresenter } from "@adapters/dtos/todo/create";
import { type RemoveTodoResponse, type RemoveTodoPresenter } from "@adapters/dtos/todo/remove";
import { type UpdateTodoResponse, type UpdateTodoPresenter } from "@adapters/dtos/todo/update";
import { type FindTodoResponse, type FindTodoPresenter } from "@adapters/dtos/todo/find";
import { type ListTodosResponse, type ListTodosPresenter } from "@adapters/dtos/todo/list";
import { type TodoView, createViewFromEntity } from "./view";
import { JsonError } from "../error";

export type CreateTodoJsonResponse = Result<TodoView, JsonError>;

export class CreateTodoJsonPresenter implements CreateTodoPresenter<CreateTodoJsonResponse> {
    present(response: CreateTodoResponse): CreateTodoJsonResponse {
        return response.map(createViewFromEntity).mapErr((err) => {
            switch (err.name) {
                case "DuplicatedTitle":
                    return JsonError.create(409, err);
                case "ParseError":
                    return JsonError.fromParse(err);
                default:
                    return JsonError.fromInternal(err);
            }
        });
    }
}

export type RemoveTodoJsonResponse = Result<void, JsonError>;

export class RemoveTodoJsonPresenter implements RemoveTodoPresenter<RemoveTodoJsonResponse> {
    present(response: RemoveTodoResponse): RemoveTodoJsonResponse {
        return response.mapErr((err) => {
            switch (err.name) {
                case "IdNotFound":
                    return JsonError.create(404, err);
                case "ParseError":
                    return JsonError.fromParse(err);
                default:
                    return JsonError.fromInternal(err);
            }
        });
    }
}

export type UpdateTodoJsonResponse = Result<void, JsonError>;

export class UpdateTodoJsonPresenter implements UpdateTodoPresenter<UpdateTodoJsonResponse> {
    present(response: UpdateTodoResponse): UpdateTodoJsonResponse {
        return response.mapErr((err) => {
            switch (err.name) {
                case "IdNotFound":
                    return JsonError.create(404, err);
                case "DuplicatedTitle":
                    return JsonError.create(409, err);
                case "ParseError":
                    return JsonError.fromParse(err);
                default:
                    return JsonError.fromInternal(err);
            }
        });
    }
}

export type FindTodoJsonResponse = Result<TodoView, JsonError>;

export class FindTodoJsonPresenter implements FindTodoPresenter<FindTodoJsonResponse> {
    present(response: FindTodoResponse): FindTodoJsonResponse {
        return response.map(createViewFromEntity).mapErr((err) => {
            switch (err.name) {
                case "IdNotFound":
                    return JsonError.create(404, err);
                case "ParseError":
                    return JsonError.fromParse(err);
                default:
                    return JsonError.fromInternal(err);
            }
        });
    }
}

export type JsonTodosList = {
    count: number;
    page: number;
    limit: number;
    data: TodoView[];
};

export type ListTodosJsonResponse = Result<JsonTodosList, JsonError>;

export class ListTodosJsonPresenter implements ListTodosPresenter<ListTodosJsonResponse> {
    present(response: ListTodosResponse): ListTodosJsonResponse {
        return response
            .map((list) => ({
                count: list.count,
                page: list.page,
                limit: list.limit,
                data: list.data.map(createViewFromEntity),
            }))
            .mapErr((err) => {
                switch (err.name) {
                    case "ParseError":
                        return JsonError.fromParse(err);
                    default:
                        return JsonError.fromInternal(err);
                }
            });
    }
}
