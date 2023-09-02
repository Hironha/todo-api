import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'

import { type TodoRepository } from '@application/repositories/todo'
import { find, FindError } from '@application/functions/todo/find'
import { type Input, type Output, OutputView } from '@adapters/dtos/todo/find'

export type InternalError = { kind: 'internal'; cause: string }
export type NotFound = {
  kind: 'not-found'
  /** describes which property was not found */
  which: string
}
export type RunError = NotFound | InternalError

export class FindController {
  constructor(private repository: TodoRepository) {}

  async run(input: View<Input>): Promise<E.Either<RunError, View<Output>>> {
    const result = await find({ repository: this.repository, input: input.view() })
    return E.mapping(result).map(OutputView.fromTodo).mapLeft(this.createError).unwrap()
  }

  private createError(error: FindError): RunError {
    switch (error) {
      case FindError.NotFound:
        return { kind: 'not-found', which: 'id' }
      case FindError.Unknown:
        return { kind: 'internal', cause: 'internal error on find todo function' }
      default:
        // exhaustive check
        return error
    }
  }
}
