import { z } from 'zod'

import { type Either } from '@core/helpers/either'
import { type ParsableInput, type ParseError } from '@core/helpers/parser'
import { ZodParser } from '@adapters/parser'
import { type TodoView } from '@adapters/views/todo'

export type ValidationError = { kind: 'validation' } & ParseError<Input>
export type InternalError = { kind: 'internal'; cause: string }
export type RunError = InternalError | ValidationError

export type Input = { title: string; description?: string; todoAt?: Date }
export type Output = Either<RunError, TodoView>

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

  parse(): Either<ParseError<Input>, Input> {
    return new ZodParser(inputSchema).parse(this.input)
  }
}
