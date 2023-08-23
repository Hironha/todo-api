import * as E from '@core/helpers/either'
import { type InternalError } from '@core/helpers/error'
import { type Todo } from '@domain/entities/todo'
import { type TodoRepository } from '@application/repositories/todo'
import * as Errors from '@application/errors/todo/create'

export type CreateInput = {
  title: string
  description: string
  todoAt?: Date
}

export type CreateContext = {
  repository: TodoRepository
  input: CreateInput
}

export async function create(ctx: CreateContext): Promise<E.Either<InternalError, Todo>> {
  try {
    const createdTodo = await ctx.repository.create(ctx.input)
    return E.right(createdTodo)
  } catch (e) {
    console.error(e)
    return E.left(Errors.general)
  }
}
