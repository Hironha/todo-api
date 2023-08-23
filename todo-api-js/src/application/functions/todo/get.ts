import * as E from '@core/helpers/either'
import { type InternalError } from '@core/helpers/error'
import { type Todo } from '@domain/entities/todo'
import { type TodoRepository } from '@application/repositories/todo'
import * as Errors from '@application/errors/todo/get'

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
