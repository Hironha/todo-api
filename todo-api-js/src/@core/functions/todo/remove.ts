import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'
import { type Todo } from '@models/todo'
import { type TodoRepository } from '@repositories/todo'
import * as Errors from '@errors/todo/delete'

export type RemoveInput = {
  id: string
}

export type RemoveContext = {
  repository: TodoRepository
  input: RemoveInput
}

export async function remove(ctx: RemoveContext): Promise<E.Either<InternalError, Todo>> {
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
