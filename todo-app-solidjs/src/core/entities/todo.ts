import {
  object,
  string,
  nullish,
  date,
  union,
  literal,
  coerce,
  safeParse,
  minLength,
  transform,
} from "valibot";

import { type Tag } from "../utils/tag";
import { type Result, Ok, Err } from "../utils/result";
import { formatDateYmd, formatDateTime, type DateYmd, type DateTime } from "../utils/date";

export type TodoId = Tag<string, "todoId">;
export type TodoTitle = Tag<string, "todoTitle">;
export type TodoStatus = "todo" | "in_progress" | "done";

export type SerializableTodo = {
  id: string;
  title: string;
  description?: string;
  todoAt?: DateYmd;
  status: TodoStatus;
  createdAt: DateTime;
  updatedAt: DateTime;
};

export type Todo = {
  id: TodoId;
  title: TodoTitle;
  description?: string;
  todoAt?: Date;
  status: TodoStatus;
  createdAt: Date;
  updatedAt: Date;
};

const TODO_TITLE_SCHEMA = string([minLength(1)]);
const TODO_STATUS_SCHEMA = union([literal("todo"), literal("in_progress"), literal("done")]);

const TODO_SCHEMA = object({
  id: string(),
  title: TODO_TITLE_SCHEMA,
  description: transform(nullish(string()), (d) => (d?.length ? d : undefined)),
  todoAt: nullish(coerce(date(), (todoAt) => new Date(todoAt as string))),
  status: TODO_STATUS_SCHEMA,
  createdAt: coerce(date(), (createdAt) => new Date(createdAt as string)),
  updatedAt: coerce(date(), (updatedAt) => new Date(updatedAt as string)),
});

export function parseTodoTitle(value: unknown): Result<TodoTitle, "length" | "string"> {
  const parsed = safeParse(TODO_TITLE_SCHEMA, value);
  if (parsed.success) {
    return new Ok(parsed.output as TodoTitle);
  }

  switch (parsed.issues[0].validation) {
    case "min_length":
      return new Err<"length">("length");
    default:
      return new Err<"string">("string");
  }
}

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

export function parseTodoStatus(value: unknown): Result<TodoStatus, undefined> {
  const parsed = safeParse(TODO_STATUS_SCHEMA, value);
  if (parsed.success) {
    return new Ok(parsed.output);
  }

  return new Err(undefined);
}

export function getSerializableTodo(todo: Todo): SerializableTodo {
  return {
    id: todo.id,
    title: todo.title,
    description: todo.description,
    todoAt: todo.todoAt ? formatDateYmd(todo.todoAt) : undefined,
    status: todo.status,
    createdAt: formatDateTime(todo.createdAt),
    updatedAt: formatDateTime(todo.updatedAt),
  };
}
