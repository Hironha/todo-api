import * as E from '@core/helpers/either'
import { type ParsableInput } from '@core/helpers/parser'

import { remove, RemoveError } from '@application/functions/todo/remove'
import { type TodoRepository } from '@application/repositories/todo'
import { type Input, type RunError, type Output } from '@adapters/dtos/todo/remove'
import { AbstractController } from '@adapters/controllers/controller'

export class RemoveController extends AbstractController<Input, Output> {
  private readonly repository: TodoRepository
  constructor(input: ParsableInput<Input>, repository: TodoRepository) {
    super(input)
    this.repository = repository
  }

  async run(): Promise<Output> {
    const input = this.input.parse()
    if (E.isLeft(input)) {
      return E.left({ kind: 'validation', details: input.value.details })
    }

    const result = await remove({ repository: this.repository, input: input.value })
    return E.map(result).mapLeft(this.mapRemoveError).unwrap()
  }

  private mapRemoveError(error: RemoveError): RunError {
    switch (error) {
      case RemoveError.NotFound:
        return { kind: 'not-found', which: 'id' }
      case RemoveError.Unknown:
        return { kind: 'internal', cause: 'Internal error on remove function' }
      default:
        // exhaustive check
        return error
    }
  }
}
