import * as E from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'
import { type TodoRepository, UpdateError as RepositoryError } from '@application/repositories/todo'

export type UpdateInput = {
  id: string
  title: string
  description?: string
  todoAt?: Date
}

export type UpdateContext = {
  repository: TodoRepository
  input: UpdateInput
}

export enum UpdateError {
  NOT_FOUND = 'NOT_FOUND',
  UNKNOWN = 'UNKNOWN',
}

export async function update(ctx: UpdateContext): Promise<E.Either<UpdateError, Todo>> {
  try {
    return E.mapping(await ctx.repository.update(ctx.input))
      .mapLeft(mapRepositoryError)
      .unwrap()
  } catch (e) {
    console.error(e)
    return E.left(UpdateError.UNKNOWN)
  }
}

function mapRepositoryError(error: RepositoryError): UpdateError {
  switch (error.kind) {
    case 'not-found':
      return UpdateError.NOT_FOUND
    case 'unknown':
      return UpdateError.UNKNOWN
    default:
      // exhaustive check
      return error
  }
}
