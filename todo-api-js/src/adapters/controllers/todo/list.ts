import * as E from '@core/helpers/either'
import { type ParsableInput, type ParseError } from '@core/helpers/parser'

import { type TodoRepository } from '@application/repositories/todo'
import { type ListOutput, list, ListError } from '@application/functions/todo/list'
import { type Output, type Input, OutputUtils } from '@adapters/dtos/todo/list'
import { AbstractController } from '@adapters/controllers/controller'

export type ValidationError = { kind: 'validation' } & ParseError<Input>
export type InternalError = { kind: 'internal'; cause: string }
export type RunError = ValidationError | InternalError

export class ListController extends AbstractController<{}, E.Either<RunError, Output>> {
  private readonly repository: TodoRepository
  constructor(input: ParsableInput<Input>, repository: TodoRepository) {
    super(input)
    this.repository = repository
  }

  async run(): Promise<E.Either<RunError, Output>> {
    const input = this.input.parse()
    if (E.isLeft(input)) {
      return E.left({ kind: 'validation', details: input.value.details })
    }

    const result = await list({ repository: this.repository })
    return E.mapping(result).map(this.createOutput).mapLeft(this.mapListError).unwrap()
  }

  private createOutput(output: ListOutput): Output {
    return { count: output.count, items: output.items.map(OutputUtils.createItemFromTodo) }
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
