import { tryMapArr } from "../utils/array";
import { type Result, Ok, Err } from "../utils/result";
import { Pagination } from "../entities/pagination";
import { type Todo, parseTodo } from "../entities/todo";

const BASE_URL = "http://localhost:8000";

export async function listTodos(): Promise<Result<Pagination<Todo>, string>> {
  const url = `${BASE_URL}/todos`;

  try {
    const response = await fetch(url, {
      headers: { method: "GET", mode: "cors", "Content-Type": "application/json" },
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
