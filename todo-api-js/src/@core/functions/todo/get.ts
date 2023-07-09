import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'
import { type Todo } from '@models/todo'
import { type TodoRepository } from '@repositories/todo'
import * as Errors from '@errors/todo/get'

export type GetInput = {
  id: string
}

export type GetContext = {
  repository: TodoRepository
  input: GetInput
}

export async function get(ctx: GetContext): Promise<E.Either<InternalError, Todo>> {
  try {
    const todo = await ctx.repository.get(ctx.input.id)
    if (!todo) {
      return E.left(Errors.notFound)
    }
    return E.right(todo)
  } catch (e) {
    console.error(e)
    return E.left(Errors.general)
  }
}
