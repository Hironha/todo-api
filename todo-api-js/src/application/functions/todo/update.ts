import * as E from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'
import {
  type TodoRepository,
  type UpdateError as RepositoryUpdateError,
} from '@application/repositories/todo'

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
    return E.map(await ctx.repository.update(ctx.input))
      .mapLeft(mapUpdateError)
      .unwrap()
  } catch (e) {
    console.error(e)
    return E.left(UpdateError.UNKNOWN)
  }
}

function mapUpdateError(error: RepositoryUpdateError): UpdateError {
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
