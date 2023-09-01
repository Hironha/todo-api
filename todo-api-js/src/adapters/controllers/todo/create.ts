import * as E from '@core/helpers/either'
import { type ApiError } from '@core/helpers/error'
import { type View } from '@core/helpers/view'

import { type TodoRepository } from '@application/repositories/todo'
import { type CreateError, CreateErrorUtils } from '@application/errors/todo/create'
import { create } from '@application/functions/todo/create'
import { type Input, type Output, OutputView } from '@adapters/dtos/todo/create'

export type InternalError = { kind: 'internal'; error: ApiError }
export type RunError = InternalError

export class CreateController {
  constructor(private repository: TodoRepository) {}

  async run(input: View<Input>): Promise<E.Either<RunError, View<Output>>> {
    const result = await create({ repository: this.repository, input: input.view() })
    return E.mapping(result).map(OutputView.fromTodo).mapLeft(this.createInternalError).unwrap()
  }

  private createInternalError(error: CreateError): InternalError {
    return {
      kind: 'internal',
      error: CreateErrorUtils.toApi(error),
    }
  }
}
