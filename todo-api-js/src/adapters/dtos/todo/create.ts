import { z } from 'zod'
import { DateUtils } from '@core/helpers/date'

import * as E from '@core/helpers/either'
import { type ParseError, ParsableInput } from '@core/helpers/parser'
import { type Todo } from '@domain/entities/todo'
import { ZodParser } from '@adapters/parser'

export type Input = {
  title: string
  description?: string
  todoAt?: Date
}

export type Output = {
  id: string
  title: string
  description?: string
  /** UTC Date stringified to `ISO-8601` YYYY-MM-DD format */
  todoAt?: string
  /** UTC Date stringified to `RFC 3339` format  */
  updatedAt: string
  /** UTC Date stringified to `RFC 3339` format  */
  createdAt: string
}

// singleton of input schema
const inputSchema = z.object({
  title: z.string({ required_error: 'title is required' }),
  description: z.string({ required_error: 'description is required' }).optional(),
  todoAt: z.coerce.date().optional(),
})

export class RawInput implements ParsableInput<Input> {
  constructor(private input: unknown) {}

  parse(): E.Either<ParseError<Input>, Input> {
    return new ZodParser(inputSchema).parse(this.input)
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
