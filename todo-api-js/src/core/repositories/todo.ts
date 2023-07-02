import { type Todo } from '@models/todo'

export type CreateInput = Pick<Todo, 'title' | 'description' | 'todoAt'>

export interface TodoRepository {
  create(input: CreateInput): Promise<Todo>
  get(id: string): Promise<Todo | undefined>
}
