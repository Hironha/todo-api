import { z } from 'zod'

import { type Either } from '@core/helpers/either'
import { ParsableInput, type ParseError } from '@core/helpers/parser'
import { ZodParser } from '@adapters/parser'

export type ValidationError = { kind: 'validation' } & ParseError<Input>
export type NotFound = { kind: 'not-found'; which: string }
export type InternalError = { kind: 'internal'; cause: string }
export type RunError = ValidationError | NotFound | InternalError

export type Input = { id: string }
export type Output = Either<RunError, void>

const inputSchema = z.object({ id: z.string() })
export class RawInput implements ParsableInput<Input> {
  constructor(private input: unknown) {}

  parse(): Either<ParseError<Input>, Input> {
    return new ZodParser(inputSchema).parse(this.input)
  }
}
