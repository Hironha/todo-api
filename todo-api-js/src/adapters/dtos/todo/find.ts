import { z } from 'zod'

import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'
import { type ApiError, type DeserializationError } from '@core/helpers/error'

import { type Todo } from '@domain/entities/todo'

const inputSchema = z.object({
  id: z.string({ required_error: 'id is required' }),
})

export type Input = { id: string }

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

  static from(input: unknown): E.Either<ApiError<DeserializationError<Input>>, InputView> {
    const result = inputSchema.safeParse(input)
    if (result.success) {
      return E.right(new InputView(result.data))
    }

    const errors = result.error.flatten().fieldErrors
    const details: DeserializationError<Input> = {}

    if (errors.id) details.id = errors.id[0]

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
