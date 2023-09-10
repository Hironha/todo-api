import * as E from '@core/helpers/either'
import { type Todo } from '@domain/entities/todo'
import {
  type CreateInput,
  type CreateError,
  type UpdateInput,
  type UpdateError,
  type FindError,
  type RemoveError,
  type ListError,
  type TodoRepository,
} from '@application/repositories/todo'
import { type TodoModel } from '@framework/models/todo'

export class TodoStore implements TodoRepository {
  private readonly store = new Map<string, TodoModel>()

  async create(input: CreateInput): Promise<E.Either<CreateError, Todo>> {
    const currentDate = new Date()
    const todo: TodoModel = {
      id: currentDate.getTime().toString(),
      title: input.title,
      description: input.description,
      todoAt: input.todoAt,
      createdAt: currentDate,
      updatedAt: currentDate,
    }
    this.store.set(todo.id, todo)
    return E.right(todo)
  }

  async find(id: string): Promise<E.Either<FindError, Todo>> {
    const model = this.store.get(id)
    return model ? E.right(model) : E.left({ kind: 'not-found' })
  }

  async list(): Promise<E.Either<ListError, Todo[]>> {
    return E.right(Array.from(this.store.values()))
  }

  async remove(id: string): Promise<E.Either<RemoveError, void>> {
    return this.store.delete(id) ? E.right(undefined) : E.left({ kind: 'not-found' })
  }

  async update(input: UpdateInput): Promise<E.Either<UpdateError, Todo>> {
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
