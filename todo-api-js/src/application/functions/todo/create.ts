import * as E from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'
import { type TodoRepository } from '@application/repositories/todo'

export type CreateInput = {
  title: string
  description?: string
  todoAt?: Date
}

export type CreateContext = {
  repository: TodoRepository
  input: CreateInput
}

export enum CreateError {
  Unknown = 'Unknown',
}

export async function create(ctx: CreateContext): Promise<E.Either<CreateError, Todo>> {
  try {
    const todo = await ctx.repository.create(ctx.input)
    if (E.isLeft(todo)) {
      return E.left(CreateError.Unknown)
    }

    return E.right(todo.value)
  } catch (e) {
    console.error(e)
    return E.left(CreateError.Unknown)
  }
}
