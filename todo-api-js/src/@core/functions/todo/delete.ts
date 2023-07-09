import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'
import { type Todo } from '@models/todo'
import { type TodoRepository } from '@repositories/todo'
import * as Errors from '@errors/todo/delete'

export type DeleteInput = {
  id: string
}

export type DeleteContext = {
  repository: TodoRepository
  input: DeleteInput
}

export async function create(ctx: DeleteContext): Promise<E.Either<InternalError, Todo>> {
  try {
    const deletedTodo = await ctx.repository.delete(ctx.input.id)
    if (!deletedTodo) {
      return E.left(Errors.notFound)
    }
    return E.right(deletedTodo)
  } catch (e) {
    console.error(e)
    return E.left(Errors.general)
  }
}
