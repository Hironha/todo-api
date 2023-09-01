import { type Todo } from '@domain/entities/todo'

export type CreateInput = Pick<Todo, 'title' | 'description' | 'todoAt'>

export interface TodoRepository {
  create(input: CreateInput): Promise<Todo>
  find(id: string): Promise<Todo | undefined>
  list(): Promise<Todo[]>
  remove(id: string): Promise<void>
}
