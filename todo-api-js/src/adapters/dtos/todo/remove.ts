import { z } from 'zod'

import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'
import { type ParseError } from '@core/helpers/parser'

import { ZodParser } from '@adapters/parser'

export type Input = { id: string }

const inputSchema = z.object({
  id: z.string(),
})

export class InputView implements View<Input> {
  constructor(private value: Input) {}

  static parse(input: Record<PropertyKey, any>): E.Either<ParseError<Input>, InputView> {
    return E.mapping(new ZodParser(inputSchema).parse(input))
      .map(i => new InputView(i))
      .unwrap()
  }

  view(): Input {
    return this.value
  }
}
