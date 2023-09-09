import { z } from 'zod'

import { type Either } from '@core/helpers/either'
import { DateUtils } from '@core/helpers/date'
import { type ParseError, type ParsableInput } from '@core/helpers/parser'

import { type Todo } from '@domain/entities/todo'
import { ZodParser } from '@adapters/parser'

export type Input = { id: string }

export type Output = {
  id: string
  title: string
  description?: string
  /** UTC Date stringified on Y-M-D format */
  todoAt?: string
  /** UTC Date stringified on `RFC 3339` format  */
  updatedAt: string
  /** UTC Date stringified on `RFC 3339` format  */
  createdAt: string
}

// singleton of input schema
const inputSchema = z.object({
  id: z.string({ required_error: 'id is required' }),
})

export class RawInput implements ParsableInput<Input> {
  constructor(private input: unknown) {}

  parse(): Either<ParseError<Input>, Input> {
    const parser = new ZodParser(inputSchema)
    return parser.parse(this.input)
  }
}

export class OutputUtils {
  static fromTodo(todo: Todo): Output {
    return {
      id: todo.id,
      title: todo.title,
      description: todo.description,
      todoAt: todo.todoAt ? DateUtils.utcYMD(todo.todoAt) : undefined,
      createdAt: DateUtils.utcRFC3339(todo.createdAt),
      updatedAt: DateUtils.utcRFC3339(todo.updatedAt),
    }
  }
}
