import { z } from 'zod'
import { DateUtils } from '@core/helpers/date'

import * as E from '@core/helpers/either'
import { type View } from '@core/helpers/view'
import { type ApiError, type DeserializationError } from '@core/helpers/error'

import { type Todo } from '@domain/entities/todo'
import { ZodParser } from '@adapters/parser'

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
    return E.mapping(new ZodParser(inputSchema).parse(input))
      .map(i => new InputView(i))
      .unwrap()
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
      todoAt: todo.todoAt ? DateUtils.utcYMD(todo.todoAt) : undefined,
      createdAt: DateUtils.utcRFC3339(todo.createdAt),
      updatedAt: DateUtils.utcRFC3339(todo.updatedAt),
    })
  }

  view(): Output {
    return this.value
  }
}
