import { type Result } from "@domain/utils/result";
import { type ListError } from "@application/repositories/todo";
import { type TodoEntity } from "@domain/entities/todo";

export type ListTodosInput = { page: number; limit: number };

export type ListTodosOutput = Result<TodoList, ListTodosError>;

export type TodoList = {
    count: number;
    page: number;
    limit: number;
    data: TodoEntity[];
};

export type ListTodosError = ListError;
