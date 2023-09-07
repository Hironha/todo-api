import { type Either } from '@core/helpers/either'

export type ParseError<From extends {}> = {
  details: {
    [P in keyof From]?: string
  }
}

export interface Parser<T extends {}> {
  parse(input: unknown): Either<ParseError<T>, T>
}

export interface ParsableInput<T extends {}> {
  parse(): Either<ParseError<T>, T>
}
