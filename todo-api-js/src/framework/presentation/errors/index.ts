import { type ParseError } from '@core/helpers/parser'

export type BasicError = {
  code: string
  message: string
}

export type DetailedError<Details extends {}> = BasicError & {
  details: Details
}

export type UnsetDetails = undefined | void | null

export type ApiError<D extends {} | UnsetDetails = undefined> = D extends {}
  ? DetailedError<D>
  : BasicError

export function createParseError<T extends {}>(error: ParseError<T>): DetailedError<ParseError<T>> {
  return { code: 'VAL-001', message: 'Validation error', details: error }
}
