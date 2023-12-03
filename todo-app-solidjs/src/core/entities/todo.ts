export type TodoStatus = "todo" | "in_progress" | "done";

export type Todo = {
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
