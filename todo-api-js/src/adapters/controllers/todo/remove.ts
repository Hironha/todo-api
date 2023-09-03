import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'

import { remove, RemoveError } from '@application/functions/todo/remove'
import { type TodoRepository } from '@application/repositories/todo'
import { Input } from '@adapters/dtos/todo/remove'

export type NotFound = {
  kind: 'not-found'
  /** describes which property was not found */
  which: string
}
export type InternalError = { kind: 'internal'; cause: string }
export type RunError = NotFound | InternalError

export class RemoveController {
  constructor(private repository: TodoRepository) {}

  async run(input: View<Input>): Promise<E.Either<RunError, void>> {
    const result = await remove({ repository: this.repository, input: input.view() })
    return E.mapping(result).mapLeft(this.createError).unwrap()
  }

  private createError(error: RemoveError): RunError {
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
