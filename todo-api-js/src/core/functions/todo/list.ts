import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'
import { type Todo } from '@models/todo'
import { type TodoRepository } from '@repositories/todo'
import * as Errors from '@errors/todo/list'

export type ListContext = {
  repository: TodoRepository
}

export async function list(ctx: ListContext): Promise<E.Either<InternalError, Todo[]>> {
  try {
    const todos = await ctx.repository.list()
    return E.right(todos)
  } catch (e) {
    console.error(e)
    return E.left(Errors.general)
  }
}
