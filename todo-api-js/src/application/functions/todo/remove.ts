import * as E from '@core/helpers/either'

import { type TodoRepository } from '@application/repositories/todo'

export type RemoveInput = {
  id: string
}

export type RemoveContext = {
  repository: TodoRepository
  input: RemoveInput
}

export enum RemoveError {
  NotFound,
  InternalError,
}

export async function remove(ctx: RemoveContext): Promise<E.Either<RemoveError, void>> {
  try {
    await ctx.repository.remove(ctx.input.id)
    return E.right(undefined)
  } catch (e) {
    console.error(e)
    return E.left(RemoveError.InternalError)
  }
}
