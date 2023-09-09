import { z } from 'zod'

import * as E from '@core/helpers/either'
import { ParsableInput, type ParseError } from '@core/helpers/parser'

import { ZodParser } from '@adapters/parser'

export type Input = { id: string }

const inputSchema = z.object({ id: z.string() })
export class RawInput implements ParsableInput<Input> {
  constructor(private input: unknown) {}

  parse(): E.Either<ParseError<Input>, Input> {
    return new ZodParser(inputSchema).parse(this.input)
  }
}
