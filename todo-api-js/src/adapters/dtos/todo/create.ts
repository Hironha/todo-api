import { z } from 'zod'

import { type Todo } from '@domain/entities/todo'
import { useZodParser } from '@adapters/parser'

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

const createInputSchema = z.object({
  title: z.string({ required_error: 'title is required' }),
  description: z.string({ required_error: 'description is required' }),
  todoAt: z.coerce.date().optional(),
})

export const parser = useZodParser(createInputSchema)

export class InputUtils {
  static parse(input: Input) {
    return parser(input)
  }
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
