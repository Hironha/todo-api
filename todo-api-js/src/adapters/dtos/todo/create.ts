import { z } from 'zod'

import * as E from '@core/helpers/either'
import { type ParsableInput, type ParseError } from '@core/helpers/parser'
import { type Todo } from '@domain/entities/todo'
import { ZodParser } from '@adapters/parser'
import { TodoViewUtils, type TodoView } from '@adapters/views/todo'

export type Input = {
  title: string
  description?: string
  todoAt?: Date
}

export type Output = TodoView & {}

// singleton of input schema
const inputSchema = z.object({
  title: z.string({ required_error: 'title is required' }),
  description: z
    .string({ required_error: 'description is required' })
    .nonempty({ message: 'if defined, description should not be empty' })
    .optional(),
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
    return TodoViewUtils.fromTodo(todo)
  }
}
