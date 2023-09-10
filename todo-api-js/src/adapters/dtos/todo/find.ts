import { z } from 'zod'

import { type Either } from '@core/helpers/either'
import { type ParseError, type ParsableInput } from '@core/helpers/parser'

import { type Todo } from '@domain/entities/todo'
import { ZodParser } from '@adapters/parser'
import { TodoViewUtils, type TodoView } from '@adapters/views/todo'

export type Input = { id: string }

export type Output = TodoView & {}

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
    return TodoViewUtils.fromTodo(todo)
  }
}
