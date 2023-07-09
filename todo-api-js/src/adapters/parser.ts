import { z } from 'zod'
import { type Parser, type ParseError, useParser } from '@helpers/parser'
import * as E from '@helpers/either'
import { InternalError } from '@helpers/error'

type ErrorFrom<T extends {}> = {
  [K in keyof T]?: string[] | undefined
}

type ZodParserError<R extends {}> = ParseError<ErrorFrom<R>>

type ParserFn<S extends z.ZodSchema> = (
  input: unknown
) => E.Either<InternalError<ZodParserError<z.infer<S>>['details']>, z.infer<S>>

class ZodParser<S extends z.ZodSchema> implements Parser<ZodParserError<z.infer<S>>, z.infer<S>> {
  constructor(private schema: S) {}

  parse(input: unknown): E.Either<ZodParserError<z.infer<S>>, z.infer<S>> {
    const result = this.schema.safeParse(input)
    if (!result.success) {
      return E.left({ message: result.error.message, details: result.error.formErrors.fieldErrors })
    }
    return E.right(result.data)
  }
}

/**
 * Adapter function to transform a zod schema into a `Parser<L, R>`. 
 */
export function useZodParser<S extends z.ZodSchema>(schema: S): ParserFn<S> {
  const parser = new ZodParser(schema)
  return (input: unknown) => {
    return useParser(parser, input)
  }
}
