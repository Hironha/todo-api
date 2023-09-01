import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'
import { type ApiError } from '@core/helpers/error'

import { remove } from '@application/functions/todo/remove'
import { type TodoRepository } from '@application/repositories/todo'
import { RemoveError, RemoveErrorUtils } from '@application/errors/todo/remove'
import { Input } from '@adapters/dtos/todo/remove'

export type NotFound = { kind: 'not-found'; error: ApiError }
export type InternalError = { kind: 'internal'; error: ApiError }
export type RunError = NotFound | InternalError

export class RemoveController {
  constructor(private repository: TodoRepository) {}

  async run(input: View<Input>): Promise<E.Either<RunError, void>> {
    const result = await remove({ repository: this.repository, input: input.view() })
    return E.mapping(result).mapLeft(this.createError).unwrap()
  }

  private createError(error: RemoveError): RunError {
    const kind: RunError['kind'] = error === RemoveError.NotFound ? 'not-found' : 'internal'
    return { kind, error: RemoveErrorUtils.toApi(error) }
  }
}
