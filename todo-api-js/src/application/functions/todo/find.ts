import * as E from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'
import { FindError } from '@application/errors/todo/find'
import { type TodoRepository } from '@application/repositories/todo'

export type FindInput = {
  id: string
}

export type FindContext = {
  repository: TodoRepository
  input: FindInput
}

export async function find(ctx: FindContext): Promise<E.Either<FindError, Todo>> {
  try {
    const todo = await ctx.repository.find(ctx.input.id)
    if (!todo) {
      return E.left(FindError.NotFound)
    }

    return E.right(todo)
  } catch (e) {
    console.error(e)
    return E.left(FindError.InternalError)
  }
}
