import { type Todo } from '@models/todo'
import { type CreateInput, type TodoRepository } from '@repositories/todo'

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

  get(id: string): Promise<Todo | undefined> {
    return Promise.resolve(this.store.get(id))
  }

  list(): Promise<Todo[]> {
    return Promise.resolve(Array.from(this.store.values()))
  }

  delete(id: string): Promise<Todo | undefined> {
    const todo = this.store.get(id)
    if (!todo) {
      return Promise.resolve(undefined)
    }

    this.store.delete(todo.id)
    return Promise.resolve(todo)
  }
}
