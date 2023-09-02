import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'

import { type TodoRepository } from '@application/repositories/todo'
import { create, CreateError } from '@application/functions/todo/create'
import { type Input, type Output, OutputView } from '@adapters/dtos/todo/create'

export type InternalError = { kind: 'internal'; cause: string }
export type RunError = InternalError

export class CreateController {
  constructor(private repository: TodoRepository) {}

  async run(input: View<Input>): Promise<E.Either<RunError, View<Output>>> {
    const result = await create({ repository: this.repository, input: input.view() })
    return E.mapping(result).map(OutputView.fromTodo).mapLeft(this.mapError).unwrap()
  }

  private mapError(error: CreateError): RunError {
    switch (error) {
      case CreateError.Unknown:
        return { kind: 'internal', cause: 'Internal error on create todo function' }
      default:
        // exhaustive check
        return error
    }
  }
}
