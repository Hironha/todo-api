import * as E from '@core/helpers/either'

export type ParseError<From extends {}> = {
  [P in keyof From]?: string
}

export interface Parser<T extends {}> {
  parse(input: unknown): E.Either<ParseError<T>, T>
}
