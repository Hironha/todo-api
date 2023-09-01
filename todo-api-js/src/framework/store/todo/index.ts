import { type Todo } from '@framework/models/todo'
import { type CreateInput, type TodoRepository } from '@application/repositories/todo'

export class TodoStore implements TodoRepository {
  private readonly store = new Map<string, Todo>()

  create(input: CreateInput): Promise<Todo> {
    const currentDate = new Date()
    const id = currentDate.getTime().toString()
    const todo: Todo = {
      id,
      title: input.title,
      description: input.description,
      todoAt: input.todoAt,
      createdAt: currentDate,
      updatedAt: currentDate,
    }
    this.store.set(id, todo)
    return Promise.resolve(todo)
  }

  find(id: string): Promise<Todo | undefined> {
    return Promise.resolve(this.store.get(id))
  }

  list(): Promise<Todo[]> {
    return Promise.resolve(Array.from(this.store.values()))
  }

  remove(id: string): Promise<Todo | undefined> {
    const todo = this.store.get(id)
    if (!todo) {
      return Promise.resolve(undefined)
    }

    const deleted = this.store.delete(todo.id)
    if (!deleted) {
      return Promise.resolve(undefined)
    }

    return Promise.resolve(todo)
  }
}
