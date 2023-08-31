import * as E from '@core/helpers/either'
import { type ApiError } from '@core/helpers/error'

import { type TodoRepository } from '@application/repositories/todo'
import { type CreateError, CreateErrorUtils } from '@application/errors/todo/create'
import { create } from '@application/functions/todo/create'
import { type Input, type Output, InputUtils, OutputUtils } from '@adapters/dtos/todo/create'

export type RunError =
  | { kind: 'internal'; error: ApiError }
  | {
      kind: 'validation'
      error: ApiError<unknown>
    }

export class CreateController {
  constructor(private repository: TodoRepository) {}

  async run(input: Input): Promise<E.Either<RunError, Output>> {
    const payload = InputUtils.parse(input)
    if (E.isLeft(payload)) {
      return E.left({ kind: 'validation', error: payload.value })
    }

    const result = await create({ repository: this.repository, input: payload.value })
    return E.mapping(result).map(OutputUtils.fromTodo).mapLeft(this.createInternalError).unwrap()
  }

  private createInternalError(error: CreateError): Extract<RunError, { kind: 'internal' }> {
    return {
      kind: 'internal',
      error: CreateErrorUtils.toInternalError(error),
    }
  }
}
