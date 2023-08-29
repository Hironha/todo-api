import * as E from '@core/helpers/either'
import { type ApiError } from '@core/helpers/error'
import { type Todo } from '@domain/entities/todo'
import { type TodoRepository } from '@application/repositories/todo'
import * as Errors from '@application/errors/todo/remove'

export type RemoveInput = {
  id: string
}

export type RemoveContext = {
  repository: TodoRepository
  input: RemoveInput
}

export async function remove(ctx: RemoveContext): Promise<E.Either<ApiError, Todo>> {
  try {
    const removedTodo = await ctx.repository.remove(ctx.input.id)
    if (!removedTodo) {
      return E.left(Errors.notFound)
    }
    return E.right(removedTodo)
  } catch (e) {
    console.error(e)
    return E.left(Errors.general)
  }
}
