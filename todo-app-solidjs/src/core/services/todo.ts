import { tryMapArr } from "../utils/array";
import { type DateYmd } from "../utils/date";
import { type Result, Ok, Err } from "../utils/result";
import { Pagination } from "../entities/pagination";
import { parseTodo, type Todo, type TodoTitle, type TodoStatus } from "../entities/todo";

const BASE_URL = "http://localhost:8000";

export type CreateTodoPayload = {
  title: TodoTitle;
  description?: string;
  status: TodoStatus;
  todoAt?: DateYmd;
};

export async function createTodo(payload: CreateTodoPayload): Promise<Result<Todo, string>> {
  const url = `${BASE_URL}/todos`;

  try {
    const response = await fetch(url, {
      method: "POST",
      mode: "cors",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    if (!response.ok) {
      return new Err("TODO: handle error");
    }

    return parseTodo(await response.json()).mapErr(() => "TODO: handle error");
  } catch (e) {
    console.log(e);
    return new Err("TODO: handle error");
  }
}

export async function listTodos(): Promise<Result<Pagination<Todo>, string>> {
  const url = `${BASE_URL}/todos`;

  try {
    const response = await fetch(url, {
      method: "GET",
      mode: "cors",
      headers: { "Content-Type": "application/json" },
    });
    if (!response.ok) {
      return new Err("TODO: handle error");
    }

    const pagination = await response.json();
    const todos = tryMapArr(pagination.items, parseTodo);
    if (todos.isErr()) {
      return new Err(todos.value[1]);
    }

    return new Ok({
      perPage: pagination.perPage,
      page: pagination.page,
      count: pagination.count,
      data: todos.value,
    });
  } catch (e) {
    return new Err("TODO: handle error");
  }
}
