import * as E from '@core/helpers/either'
import { type ParseError, type ParsableInput } from '@core/helpers/parser'

import { type TodoRepository } from '@application/repositories/todo'
import { update, UpdateError } from '@application/functions/todo/update'
import { AbstractController } from '@adapters/controllers/controller'
import { OutputUtils, type Input, type Output } from '@adapters/dtos/todo/update'

export type ValidationError = { kind: 'validation' } & ParseError<Input>
export type InternalError = { kind: 'internal'; cause: string }
export type NotFoundError = { kind: 'not-found'; which: string }
export type RunError = InternalError | ValidationError | NotFoundError

export class UpdateController extends AbstractController<Input, E.Either<RunError, Output>> {
  private readonly repository: TodoRepository
  constructor(input: ParsableInput<Input>, repository: TodoRepository) {
    super(input)
    this.repository = repository
  }

  async run(): Promise<E.Either<RunError, Output>> {
    const input = this.input.parse()
    if (E.isLeft(input)) {
      return E.left({ kind: 'validation', details: input.value.details })
    }

    const result = await update({ repository: this.repository, input: input.value })
    return E.mapping(result).map(OutputUtils.fromTodo).mapLeft(this.mapUpdateError).unwrap()
  }

  private mapUpdateError(error: UpdateError): RunError {
    switch (error) {
      case UpdateError.NOT_FOUND:
        return { kind: 'not-found', which: 'id' }
      case UpdateError.UNKNOWN:
        return { kind: 'internal', cause: 'Internal error on create todo function' }
      default:
        // exhaustive check
        return error
    }
  }
}
