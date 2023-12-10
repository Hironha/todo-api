import { type Result, Ok, Err } from "../utils/result";
import { Pagination } from "../entities/pagination";
import { type Todo, TodoUtils } from "../entities/todo";
import { ArrayUtils } from "../utils/array";

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
      const todos = ArrayUtils.tryMap(pagination.items, TodoUtils.parse);
      if (todos.isErr()) {
        return todos;
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
