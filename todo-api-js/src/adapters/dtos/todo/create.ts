import { z } from 'zod'

import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'
import { type ApiError, type DeserializationError } from '@core/helpers/error'

import { type Todo } from '@domain/entities/todo'
import { type CreateInput } from '@application/functions/todo/create'

const inputSchema = z.object({
  title: z.string({ required_error: 'title is required' }),
  description: z.string({ required_error: 'description is required' }),
  todoAt: z.coerce.date().optional(),
})

export type Input = {
  title: string
  description: string
  todoAt?: Date
}

export type Output = {
  id: string
  title: string
  description: string
  /** UTC Date stringified on Y-M-D format */
  todoAt?: string
  /** UTC Date stringified on `RFC 3339` format  */
  updatedAt: string
  /** UTC Date stringified on `RFC 3339` format  */
  createdAt: string
}

export class InputView implements View<Input> {
  protected constructor(private input: Input) {}

  static parse(input: unknown): E.Either<ApiError<DeserializationError<Input>>, InputView> {
    const result = inputSchema.safeParse(input)
    if (result.success) {
      return E.right(new InputView(result.data))
    }

    const errors = result.error.flatten().fieldErrors
    const details: DeserializationError<CreateInput> = {}

    if (errors.title) details.title = errors.title[0]
    if (errors.todoAt) details.todoAt = errors.todoAt[0]
    if (errors.description) details.description = errors.description[0]

    return E.left({ code: 'VAL-001', message: 'validation error', details })
  }

  view(): Input {
    return this.input
  }
}

export class OutputView implements View<Output> {
  constructor(private value: Output) {}

  static fromTodo(todo: Todo): OutputView {
    return new OutputView({
      id: todo.id,
      title: todo.title,
      description: todo.description,
      todoAt: todo.todoAt?.toUTCString(),
      createdAt: todo.createdAt.toUTCString(),
      updatedAt: todo.updatedAt.toUTCString(),
    })
  }

  view(): Output {
    return this.value
  }
}
