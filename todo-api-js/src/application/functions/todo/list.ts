import * as E from '@core/helpers/either'

import { type Todo } from '@domain/entities/todo'
import { type TodoRepository } from '@application/repositories/todo'
import { ListError } from '@application/errors/todo/list'

export type ListContext = {
  repository: TodoRepository
}

export type ListOutput = {
  count: number
  items: Todo[]
}

export async function list(ctx: ListContext): Promise<E.Either<ListError, ListOutput>> {
  try {
    const todos = await ctx.repository.list()
    return E.right({ count: todos.length, items: todos })
  } catch (e) {
    console.error(e)
    return E.left(ListError.Internal)
  }
}
