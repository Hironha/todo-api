import * as E from '@core/helpers/either'
import { type ApiError } from '@core/helpers/error'

export type ParseError<From extends {}> = {
  [P in keyof From]?: string
}

export interface Parser<T extends {}> {
  parse(input: unknown): E.Either<ApiError<ParseError<T>>, T>
}
