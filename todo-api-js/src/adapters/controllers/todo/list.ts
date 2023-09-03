import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'

import { type TodoRepository } from '@application/repositories/todo'
import { type ListOutput, list, ListError } from '@application/functions/todo/list'
import { OutputView, type Output } from '@adapters/dtos/todo/list'

export type InternalError = { kind: 'internal'; cause: string }
export type RunError = InternalError

export class ListController {
  constructor(private repository: TodoRepository) {}

  async run(): Promise<E.Either<RunError, View<Output>>> {
    const result = await list({ repository: this.repository })
    return E.mapping(result).map(this.createOutput).mapLeft(this.createError).unwrap()
  }

  private createOutput(output: ListOutput): OutputView {
    return new OutputView().setItemsFromTodos(output.items).setCount(output.count)
  }

  private createError(error: ListError): RunError {
    switch (error) {
      case ListError.Unknown:
        return { kind: 'internal', cause: 'Internal error on list todos' }
      default:
        // exhaustive check
        return error
    }
  }
}
