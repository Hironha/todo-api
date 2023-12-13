import { object, string, nullish, date, union, literal, coerce, safeParse } from "valibot";

import { formatDateYmd } from "../utils/date";
import { type Result, Ok, Err } from "../utils/result";

export type TodoStatus = "todo" | "in_progress" | "done";

export type SerializableTodo = {
  id: string;
  title: string;
  description?: string;
  /** Date in ISO 8601 YYYY-MM-DD format */
  todoAt?: string;
  status: TodoStatus;
  /** Date in RFC 3339 format, i.e date time */
  createdAt: string;
  /** Date in RFC 3339 format, i.e date time */
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

const TODO_SCHEMA = object({
  id: string(),
  title: string(),
  description: nullish(string()),
  todoAt: nullish(coerce(date(), (todoAt) => new Date(todoAt as string))),
  status: union([literal("todo"), literal("in_progress"), literal("done")]),
  createdAt: coerce(date(), (createdAt) => new Date(createdAt as string)),
  updatedAt: coerce(date(), (updatedAt) => new Date(updatedAt as string)),
});

// TODO: return a `Result` with an error that allows identification
// of the field that failed parsing
export function parseTodo(value: unknown): Result<Todo, [keyof Todo, string]> {
  const parsed = safeParse(TODO_SCHEMA, value);
  if (parsed.success) {
    return new Ok(parsed.output as Todo);
  }

  const firstIssue = parsed.issues[0];
  const error: [keyof Todo, string] = [firstIssue.origin as keyof Todo, firstIssue.message];
  return new Err(error);
}

export function getSerializableTodo(todo: Todo): SerializableTodo {
  return {
    id: todo.id,
    title: todo.title,
    description: todo.description,
    todoAt: todo.todoAt ? formatDateYmd(todo.todoAt) : undefined,
    status: todo.status,
    createdAt: todo.createdAt.toISOString(),
    updatedAt: todo.updatedAt.toISOString(),
  };
}
