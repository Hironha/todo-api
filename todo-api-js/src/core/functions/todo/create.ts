import * as E from '@helpers/either'
import { type RequestError } from '@helpers/error'
import { Todo } from '@models/todo'
import { TodoRepository } from '@repositories/todo'

export type CreateInput = {
  title: string
  description: string
  todoAt?: Date
}

export type CreateOutput = E.Either<RequestError, Todo>

export type CreateContext = {
  repository: TodoRepository
  input: CreateInput
}

export async function create({ input, repository }: CreateContext): Promise<CreateOutput> {
  try {
    const createdTodo = await repository.create({
      title: input.title,
      todoAt: input.todoAt,
      description: input.description,
    })
    return E.right(createdTodo)
  } catch (e) {
    console.error(e)
    return E.left({ code: 'todo', message: 'todo', shortMessage: 'todo' })
  }
}
