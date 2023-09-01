import * as E from '@core/helpers/either'
import { type ApiError } from '@core/helpers/error'

import { type TodoRepository } from '@application/repositories/todo'
import { type ListOutput, list } from '@application/functions/todo/list'
import { ListError, ListErrorUtils } from '@application/errors/todo/list'
import { OutputView } from '@adapters/dtos/todo/list'

export type InternalError = { kind: 'internal'; error: ApiError }
export type RunError = InternalError

export class ListController {
  constructor(private repository: TodoRepository) {}

  async run(): Promise<E.Either<RunError, OutputView>> {
    const result = await list({ repository: this.repository })
    return E.mapping(result).map(this.createOutput).mapLeft(this.createError).unwrap()
  }

  private createOutput(output: ListOutput): OutputView {
    return new OutputView().setItemsFromTodos(output.items).setCount(output.count)
  }

  private createError(error: ListError): RunError {
    const kind: RunError['kind'] = 'internal'
    return { kind, error: ListErrorUtils.toApi(error) }
  }
}
