import * as E from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'
import {
  type FindError as RepositoryFindError,
  type TodoRepository,
} from '@application/repositories/todo'

export type FindInput = {
  id: string
}

export type FindContext = {
  repository: TodoRepository
  input: FindInput
}

export enum FindError {
  NotFound = 'NotFound',
  Unknown = 'Unknown',
}

export async function find(ctx: FindContext): Promise<E.Either<FindError, Todo>> {
  try {
    const todo = await ctx.repository.find(ctx.input.id)
    return E.map(todo).mapLeft(mapFindError).unwrap()
  } catch (e) {
    console.error(e)
    return E.left(FindError.Unknown)
  }
}

function mapFindError(error: RepositoryFindError): FindError {
  switch (error.kind) {
    case 'not-found':
      return FindError.NotFound
    case 'unknown':
      return FindError.Unknown
    default:
      // exhaustive check
      return error
  }
}
