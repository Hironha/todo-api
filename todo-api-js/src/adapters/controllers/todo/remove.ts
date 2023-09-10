import * as E from '@core/helpers/either'
import { type ParsableInput, type ParseError } from '@core/helpers/parser'

import { remove, RemoveError } from '@application/functions/todo/remove'
import { type TodoRepository } from '@application/repositories/todo'
import { Input } from '@adapters/dtos/todo/remove'
import { AbstractController } from '@adapters/controllers/controller'

export type ValidationError = { kind: 'validation' } & ParseError<Input>
export type NotFound = { kind: 'not-found'; which: string }
export type InternalError = { kind: 'internal'; cause: string }
export type RunError = ValidationError | NotFound | InternalError

export class RemoveController extends AbstractController<Input, E.Either<RunError, void>> {
  private readonly repository: TodoRepository
  constructor(input: ParsableInput<Input>, repository: TodoRepository) {
    super(input)
    this.repository = repository
  }

  async run(): Promise<E.Either<RunError, void>> {
    const input = this.input.parse()
    if (E.isLeft(input)) {
      return E.left({ kind: 'validation', details: input.value.details })
    }

    const result = await remove({ repository: this.repository, input: input.value })
    return E.mapping(result).mapLeft(this.mapRemoveError).unwrap()
  }

  private mapRemoveError(error: RemoveError): RunError {
    switch (error) {
      case RemoveError.NotFound:
        return { kind: 'not-found', which: 'id' }
      case RemoveError.InternalError:
        return { kind: 'internal', cause: 'Internal error on remove function' }
      default:
        // exhaustive check
        return error
    }
  }
}
