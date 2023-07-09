import { type Todo } from '@models/todo'
import { type CreateInput, type TodoRepository } from '@repositories/todo'

export class TodoStore implements TodoRepository {
  private readonly store = new Map<string, Todo>()

  create(input: CreateInput): Promise<Todo> {
    const currentDate = new Date()
    const id = currentDate.getTime().toString()
    const todo: Todo = {
      ...input,
      id,
      createdAt: currentDate,
      updatedAt: currentDate,
    }
    this.store.set(id, todo)
    return Promise.resolve(todo)
  }

  get(id: string): Promise<Todo | undefined> {
    return Promise.resolve(this.store.get(id))
  }

  list(): Promise<Todo[]> {
    return Promise.resolve(Array.from(this.store.values()))
  }
}
