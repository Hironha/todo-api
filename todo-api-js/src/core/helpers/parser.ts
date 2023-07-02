import * as E from '@helpers/either'
import { InternalError } from '@helpers/error'

export type ParseError<D> = { message: string; details: D }

export interface Parser<L extends ParseError<any>, R> {
  parse(input: unknown): E.Either<L, R>
}

export function useParser<L extends ParseError<any>, R>(
  parser: Parser<L, R>,
  input: unknown
): E.Either<InternalError<unknown>, R> {
  const result = parser.parse(input)
  if (E.isLeft(result)) {
    return E.left({
      code: 'VAL-001',
      message: result.value.message,
      shortMessage: 'ValidationError',
      details: result.value.details,
    })
  }
  return E.right(result.value)
}
