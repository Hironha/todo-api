import { object, string, nullish, date, union, literal, safeParse } from "valibot";
import { DateUtils } from "../utils/date";

export type TodoStatus = "todo" | "in_progress" | "done";

export type SerializableTodo = {
  id: string;
  title: string;
  description?: string;
  /** Date in ISO 8601 `YYYY-MM-DD` format */
  todoAt?: string;
  status: TodoStatus;
  /** Date in RFC 3339 format, i.e date time */
  createdAt: string;
  /** Date in RFC 339 format, i.e date time */
  updatedAt: string;
};

export type Todo = {
  id: string;
  title: string;
  description?: string;
  todoAt?: Date;
  status: TodoStatus;
  createdAt: Date;
  updatedAt: Date;
};

export class TodoUtils {
  static parse(value: unknown): Todo | undefined {
    const parser = new TodoParser();
    return parser.parse(value);
  }

  static serializable(todo: Todo): SerializableTodo {
    return {
      id: todo.id,
      title: todo.title,
      description: todo.description,
      todoAt: todo.todoAt ? DateUtils.toYmd(todo.todoAt) : undefined,
      status: todo.status,
      createdAt: todo.createdAt.toISOString(),
      updatedAt: todo.updatedAt.toISOString(),
    };
  }
}

class TodoParser {
  private readonly parser = object({
    id: string(),
    title: string(),
    description: nullish(string()),
    todoAt: nullish(date()),
    status: union([literal("todo"), literal("in_progress"), literal("done")]),
    createdAt: date(),
    updatedAt: date(),
  });

  parse(value: unknown): Todo | undefined {
    const parsed = safeParse(this.parser, value);
    if (parsed.success) {
      return parsed.output as Todo;
    } else {
      return undefined;
    }
  }
}
