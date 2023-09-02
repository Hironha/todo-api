import { z } from 'zod'
import * as E from '@core/helpers/either'
import { type Parser, type ParseError } from '@core/helpers/parser'

type Entry<T extends {}> = [keyof T, T[keyof T]]

export class ZodParser<S extends z.ZodSchema> implements Parser<z.infer<S>> {
  constructor(private schema: S) {}

  parse(input: unknown): E.Either<ParseError<z.TypeOf<S>>, z.TypeOf<S>> {
    const result = this.schema.safeParse(input)
    if (result.success) {
      return E.right(result.data)
    }

    const errors = result.error.flatten().fieldErrors
    const details: ParseError<z.infer<S>> = {}
    Object.entries(errors).forEach(([k, v]: Entry<z.infer<S>>) => {
      const detail = v?.at(0)
      if (detail) {
        details[k] = detail
      }
    })

    return E.left({ details })
  }
}
