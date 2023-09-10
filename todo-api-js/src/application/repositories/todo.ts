import { type Either } from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'

export type CreateInput = { title: string; description?: string; todoAt?: Date }
export type UpdateInput = { id: string; title: string; description?: string; todoAt?: Date }

export type NotFoundError = { kind: 'not-found' }
export type UnknownError = { kind: 'unknown' }
export type UpdateError = NotFoundError | UnknownError

export interface TodoRepository {
  create(input: CreateInput): Promise<Todo>
  find(id: string): Promise<Todo | undefined>
  list(): Promise<Todo[]>
  remove(id: string): Promise<void>
  update(input: UpdateInput): Promise<Either<UpdateError, Todo>>
}
