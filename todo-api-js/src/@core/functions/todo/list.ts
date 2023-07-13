import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'
import { type Todo } from '@models/todo'
import { type TodoRepository } from '@repositories/todo'
import * as Errors from '@errors/todo/list'

export type ListContext = {
  repository: TodoRepository
}

export type ListOutput = {
  count: number
  items: Todo[]
}

export async function list(ctx: ListContext): Promise<E.Either<InternalError, ListOutput>> {
  try {
    const todos = await ctx.repository.list()
    return E.right({ count: todos.length, items: todos })
  } catch (e) {
    console.error(e)
    return E.left(Errors.general)
  }
}
