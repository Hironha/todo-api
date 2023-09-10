import { type Either, right, left } from '@core/helpers/either'
import { type ParseError, type ParsableInput } from '@core/helpers/parser'
import { type TodoView } from '@adapters/views/todo'

export type ValidationError = { kind: 'validation' } & ParseError<Input>
export type InternalError = { kind: 'internal'; cause: string }
export type RunError = ValidationError | InternalError
export type OutputData = { count: number; items: TodoView[] }

export type Input = {}
export type Output = Either<RunError, OutputData>

export class RawInput implements ParsableInput<Input> {
  constructor(private readonly input: unknown) {}

  parse(): Either<ParseError<Input>, Input> {
    if (typeof this.input === 'object' && !!this.input) {
      return right(this.input)
    }
    return left({})
  }
}
