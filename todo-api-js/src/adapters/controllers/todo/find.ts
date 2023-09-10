import * as E from '@core/helpers/either'
import { type ParsableInput } from '@core/helpers/parser'

import { type TodoRepository } from '@application/repositories/todo'
import { find, FindError } from '@application/functions/todo/find'
import { AbstractController } from '@adapters/controllers/controller'
import { type Input, type Output, type RunError } from '@adapters/dtos/todo/find'
import { TodoViewUtils } from '@adapters/views/todo'

export class FindController extends AbstractController<Input, Output> {
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

    const result = await find({ repository: this.repository, input: input.value })
    return E.map(result).mapRight(TodoViewUtils.fromTodo).mapLeft(this.mapFindError).unwrap()
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
