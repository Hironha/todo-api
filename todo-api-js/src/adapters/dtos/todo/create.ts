import { type Todo } from '@domain/entities/todo'

export type Input = Record<PropertyKey, unknown>

export type Output = {
  id: string
  title: string
  description: string
  /** UTC Date stringified on Y-M-D format */
  todoAt?: string
  /** UTC Date stringified on `RFC 3339` format  */
  updatedAt: string
  /** UTC Date stringified on `RFC 3339` format  */
  createdAt: string
}

export class OutputUtils {
  static fromTodo(todo: Todo): Output {
    return {
      id: todo.id,
      title: todo.title,
      description: todo.description,
      todoAt: todo.todoAt?.toUTCString(),
      createdAt: todo.createdAt.toUTCString(),
      updatedAt: todo.updatedAt.toUTCString(),
    }
  }
}
