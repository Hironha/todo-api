import * as E from '@core/helpers/either'
import { type ParsableInput } from '@core/helpers/parser'

import { type TodoRepository } from '@application/repositories/todo'
import { list, ListError } from '@application/functions/todo/list'
import { type Output, type Input, type RunError } from '@adapters/dtos/todo/list'
import { AbstractController } from '@adapters/controllers/controller'
import { TodoViewUtils } from '@adapters/views/todo'

export class ListController extends AbstractController<{}, Output> {
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

    const result = await list({ repository: this.repository })
    if (E.isLeft(result)) {
      return E.left(this.mapListError(result.value))
    }

    return E.right({
      count: result.value.count,
      items: result.value.items.map(TodoViewUtils.fromTodo),
    })
  }

  private mapListError(error: ListError): RunError {
    switch (error) {
      case ListError.Unknown:
        return { kind: 'internal', cause: 'Internal error on list todos' }
      default:
        // exhaustive check
        return error
    }
  }
}
