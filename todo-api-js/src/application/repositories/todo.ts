import { type Either } from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'

export type CreateInput = { title: string; description?: string; todoAt?: Date }
export type UpdateInput = { id: string; title: string; description?: string; todoAt?: Date }

export type NotFoundError = { kind: 'not-found' }
export type UnknownError = { kind: 'unknown' }

export type CreateError = UnknownError
export type FindError = NotFoundError | UnknownError
export type RemoveError = NotFoundError | UnknownError
export type ListError = UnknownError
export type UpdateError = NotFoundError | UnknownError

export interface TodoRepository {
  create(input: CreateInput): Promise<Either<CreateError, Todo>>
  find(id: string): Promise<Either<FindError, Todo>>
  list(): Promise<Either<ListError, Todo[]>>
  remove(id: string): Promise<Either<RemoveError, void>>
  update(input: UpdateInput): Promise<Either<UpdateError, Todo>>
}
