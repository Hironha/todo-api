import * as E from '@core/helpers/either'
import { type ApiError } from '@core/helpers/error'
import { type View } from '@core/helpers/view'

import { type TodoRepository } from '@application/repositories/todo'
import { FindError, FindErrorUtils } from '@application/errors/todo/find'
import { find } from '@application/functions/todo/find'
import { type Input, type Output, OutputView } from '@adapters/dtos/todo/find'

export type NotFound = { kind: 'not-found'; error: ApiError }
export type InternalError = { kind: 'internal'; error: ApiError }
export type RunError = NotFound | InternalError

export class FindController {
  constructor(private repository: TodoRepository) {}

  async run(input: View<Input>): Promise<E.Either<RunError, View<Output>>> {
    const result = await find({ repository: this.repository, input: input.view() })
    return E.mapping(result).map(OutputView.fromTodo).mapLeft(this.createError).unwrap()
  }

  private createError(error: FindError): RunError {
    let kind: RunError['kind'] = error === FindError.Internal ? 'internal' : 'not-found'
    return { kind, error: FindErrorUtils.toApi(error) }
  }
}
