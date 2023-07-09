import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'

export type ParseError<D> = { message: string; details: D }

export interface Parser<L extends ParseError<any>, R> {
  parse(input: unknown): E.Either<L, R>
}

/**
 * This functions is responsible for transforming any result of a `Parser<L, R>`
 * into an internal validation. In case of error, it's going to return an `InternalError<unknown>`,
 * otherwise, it's going to return the parsed value `R`.
 */
export function useParser<L extends ParseError<any>, R>(
  parser: Parser<L, R>,
  input: unknown
): E.Either<InternalError<L['details']>, R> {
  const result = parser.parse(input)
  if (E.isLeft(result)) {
    return E.left({
      code: 'VAL-001',
      message: result.value.message,
      shortMessage: 'ValidationError',
      details: result.value.details,
    }) as E.Either<InternalError<L['details']>, R>
  }
  return E.right(result.value)
}
