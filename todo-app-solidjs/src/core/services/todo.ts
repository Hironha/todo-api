import { type Result, Ok, Err } from "../utils/result";
import { Pagination } from "../entities/pagination";
import { type Todo, TodoUtils } from "../entities/todo";
import { ArrayUtils } from "../utils/array";

function tryParseTodo(value: unknown): Result<Todo, undefined> {
  const todo = TodoUtils.parse(value);
  return todo ? new Ok(todo) : new Err(undefined);
}

export class TodoService {
  static async list(): Promise<Result<Pagination<Todo>, string>> {
    const url = `http://localhost:8000/todos`;

    try {
      const response = await fetch(url, {
        headers: { method: "GET", mode: "cors", "Content-Type": "application/json" },
      });
      if (!response.ok) {
        return new Err("TODO: handle error");
      }

      const pagination = await response.json();

      const todos = ArrayUtils.tryMap(pagination.items, tryParseTodo);
      if (todos.isErr()) {
        return new Err("TODO: handle error");
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
}
