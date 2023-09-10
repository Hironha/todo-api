import { DateUtils } from '@core/helpers/date'
import { type Todo } from '@domain/entities/todo'

/** Presentable format of `Todo` entity */
export type TodoView = {
  id: string
  title: string
  description?: string
  /** UTC Date stringified to `ISO-8601` on YYYY-MM-DD format */
  todoAt?: string
  /** UTC Date stringified to `RFC 3339` format  */
  updatedAt: string
  /** UTC Date stringified to `RFC 3339` format  */
  createdAt: string
}

export class TodoViewUtils {
  static fromTodo(todo: Todo): TodoView {
    const view: TodoView = {
      id: todo.id,
      title: todo.title,
      createdAt: DateUtils.utcRFC3339(todo.createdAt),
      updatedAt: DateUtils.utcRFC3339(todo.updatedAt),
    }

    if (todo.description !== undefined) view.description = todo.description
    if (todo.todoAt) view.todoAt = DateUtils.utcYMD(todo.todoAt)

    return view
  }
}
