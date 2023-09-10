import * as E from '@core/helpers/either'
import { type ParseError, type ParsableInput } from '@core/helpers/parser'

import { type TodoRepository } from '@application/repositories/todo'
import { find, FindError } from '@application/functions/todo/find'
import { AbstractController } from '@adapters/controllers/controller'
import { OutputUtils, type Input, type Output } from '@adapters/dtos/todo/find'

export type ValidationError = { kind: 'validation' } & ParseError<Input>
export type InternalError = { kind: 'internal'; cause: string }
export type NotFoundError = { kind: 'not-found'; which: string }
export type RunError = InternalError | ValidationError | NotFoundError

export class FindController extends AbstractController<Input, E.Either<RunError, Output>> {
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

    const result = await find({ repository: this.repository, input: input.value })
    return E.mapping(result).map(OutputUtils.fromTodo).mapLeft(this.mapFindError).unwrap()
  }

  private mapFindError(error: FindError): RunError {
    switch (error) {
      case FindError.NotFound:
        return { kind: 'not-found', which: 'id' }
      case FindError.Unknown:
        return { kind: 'internal', cause: 'Internal error on create todo function' }
      default:
        // exhaustive check
        return error
    }
  }
}
