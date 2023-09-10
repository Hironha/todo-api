import * as E from '@core/helpers/either'

import { type Todo } from '@domain/entities/todo'
import {
  type ListError as RepositoryListError,
  type TodoRepository,
} from '@application/repositories/todo'

export type ListOutput = { count: number; items: Todo[] }
export enum ListError {
  Unknown,
}

export type ListContext = {
  repository: TodoRepository
}

export async function list(ctx: ListContext): Promise<E.Either<ListError, ListOutput>> {
  try {
    const todos = await ctx.repository.list()
    return E.map(todos).mapRight(createOutput).mapLeft(mapListError).unwrap()
  } catch (e) {
    console.error(e)
    return E.left(ListError.Unknown)
  }
}

function createOutput(todos: Todo[]): ListOutput {
  return { count: todos.length, items: todos }
}

function mapListError(error: RepositoryListError): ListError {
  switch (error.kind) {
    case 'unknown':
      return ListError.Unknown
    default:
      // exhaustive check
      return error.kind
  }
}
