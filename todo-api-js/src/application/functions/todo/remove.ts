import * as E from '@core/helpers/either'

import {
  type RemoveError as RepositoryRemoveError,
  type TodoRepository,
} from '@application/repositories/todo'

export type RemoveInput = { id: string }
export type RemoveContext = {
  repository: TodoRepository
  input: RemoveInput
}

export enum RemoveError {
  NotFound,
  Unknown,
}

export async function remove(ctx: RemoveContext): Promise<E.Either<RemoveError, void>> {
  try {
    const result = await ctx.repository.remove(ctx.input.id)
    return E.map(result).mapLeft(mapRemoveError).unwrap()
  } catch (e) {
    console.error(e)
    return E.left(RemoveError.Unknown)
  }
}

function mapRemoveError(error: RepositoryRemoveError): RemoveError {
  switch (error.kind) {
    case 'not-found':
      return RemoveError.NotFound
    case 'unknown':
      return RemoveError.Unknown
    default:
      // exhaustive check
      return error
  }
}
