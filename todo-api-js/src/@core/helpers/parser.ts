import * as E from '@core/helpers/either'
import { type InternalError } from '@core/helpers/error'

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
  return E.mapping(parser.parse(input)).mapLeft(toInternalError).unwrap()
}

function toInternalError<E extends ParseError<any>>(err: E): InternalError<E['details']> {
  return {
    code: 'VAL-001',
    message: err.message,
    details: err.details,
  } as InternalError<E['details']>
}
