import { z } from 'zod'
import { type Parser, type ParseError, useParser } from '@core/helpers/parser'
import * as E from '@core/helpers/either'
import { ApiError } from '@core/helpers/error'

type ErrorFrom<T extends {}> = {
  [K in keyof T]?: string[] | undefined
}

type ZodParserError<R extends {}> = ParseError<ErrorFrom<R>>

type ParserFn<S extends z.ZodSchema> = (
  input: unknown
) => E.Either<ApiError<ZodParserError<z.infer<S>>['details']>, z.infer<S>>

class ZodParser<S extends z.ZodSchema> implements Parser<ZodParserError<z.infer<S>>, z.infer<S>> {
  constructor(private schema: S) {}

  parse(input: unknown): E.Either<ZodParserError<z.infer<S>>, z.infer<S>> {
    const result = this.schema.safeParse(input)
    if (!result.success) {
      const firstMessage = Object.values(result.error.flatten().fieldErrors).at(0)?.at(0)
      return E.left({
        message: firstMessage ?? 'Validation error',
        details: result.error.formErrors.fieldErrors,
      })
    }
    return E.right(result.data)
  }
}

/**
 * Adapter function to transform a zod schema into a `Parser<L, R>`.
 */
export function useZodParser<S extends z.ZodSchema>(schema: S): ParserFn<S> {
  const parser = new ZodParser(schema)
  return (input: unknown) => useParser(parser, input)
}
