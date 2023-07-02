import { type Todo } from '@models/todo'
import { type CreateInput, type TodoRepository } from '@repositories/todo'

export class TodoStore implements TodoRepository {
  private readonly store = new Set<Todo>()

  create(input: CreateInput): Promise<Todo> {
    const currentDate = new Date()
    const todo: Todo = {
      ...input,
      id: currentDate.getTime().toString(),
      createdAt: currentDate,
      updatedAt: currentDate,
    }
    this.store.add(todo)
    return Promise.resolve(todo)
  }
}
