import * as E from '@core/helpers/either'
import {
  UpdateError,
  UpdateInput,
  type CreateInput,
  type TodoRepository,
} from '@application/repositories/todo'
import { type TodoModel } from '@framework/models/todo'

export class TodoStore implements TodoRepository {
  private readonly store = new Map<string, TodoModel>()

  async create(input: CreateInput): Promise<TodoModel> {
    const currentDate = new Date()
    const id = currentDate.getTime().toString()
    const todo: TodoModel = {
      id,
      title: input.title,
      description: input.description,
      todoAt: input.todoAt,
      createdAt: currentDate,
      updatedAt: currentDate,
    }
    this.store.set(id, todo)
    return todo
  }

  async find(id: string): Promise<TodoModel | undefined> {
    return this.store.get(id)
  }

  async list(): Promise<TodoModel[]> {
    return Array.from(this.store.values())
  }

  async remove(id: string): Promise<void> {
    this.store.delete(id)
  }

  async update(input: UpdateInput): Promise<E.Either<UpdateError, TodoModel>> {
    const todo = this.store.get(input.id)
    if (!todo) {
      return E.left({ kind: 'not-found' })
    }

    const updated: TodoModel = {
      id: todo.id,
      title: input.title,
      description: input.description,
      todoAt: input.todoAt,
      createdAt: todo.createdAt,
      updatedAt: new Date(),
    }
    this.store.set(input.id, todo)
    return E.right(updated)
  }
}
