import { Result } from "../utils/result";
import { Pagination } from "../entities/pagination";
import { type Todo } from "../entities/todo";

export class TodoService {
  private baseUrl = "http://localhost:8000";

  async list(signal?: AbortSignal): Promise<Result<Pagination<Todo>, string>> {
    const url = `${this.baseUrl}/todos`;
    const response = await fetch(url, {
      headers: { method: "GET", mode: "cors", "Content-Type": "application/json" },
      signal,
    });

    if (response.ok) {
      return Result.ok(await response.json());
    } else {
      return Result.err("TODO: handle error");
    }
  }
}
