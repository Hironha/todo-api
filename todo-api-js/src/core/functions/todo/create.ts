import { Todo } from '@models/todo'
import { TodoRepository } from 'src/core/repositories/todo'

export type CreateInput = {
  title: string
  description: string
  todoAt?: Date
}

export type CreateContext = {
  repository: TodoRepository
  input: CreateInput
}

export async function create({ input, repository }: CreateContext): Promise<Todo | null> {
  try {
    return await repository.create({
      title: input.title,
      todoAt: input.todoAt,
      description: input.description,
    })
  } catch (e) {
    console.error(e)
    return null
  }
}
