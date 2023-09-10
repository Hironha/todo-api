import * as E from '@core/helpers/either'
import { type ParsableInput } from '@core/helpers/parser'

import { type TodoRepository } from '@application/repositories/todo'
import { create, CreateError } from '@application/functions/todo/create'
import { AbstractController } from '@adapters/controllers/controller'
import { TodoViewUtils } from '@adapters/views/todo'
import { type Input, type Output, type RunError } from '@adapters/dtos/todo/create'

export class CreateController extends AbstractController<Input, Output> {
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

    const result = await create({ repository: this.repository, input: input.value })
    return E.map(result).mapRight(TodoViewUtils.fromTodo).mapLeft(this.mapCreateError).unwrap()
  }

  private mapCreateError(error: CreateError): RunError {
    switch (error) {
      case CreateError.Unknown:
        return { kind: 'internal', cause: 'Internal error on create todo function' }
      default:
        // exhaustive check
        return error
    }
  }
}
