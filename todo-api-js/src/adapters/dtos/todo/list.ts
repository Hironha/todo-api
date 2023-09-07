import { type Either, right, left } from '@core/helpers/either'
import { type ParseError, type ParsableInput } from '@core/helpers/parser'
import { DateUtils } from '@core/helpers/date'
import { type Todo } from '@domain/entities/todo'

export type Input = {}
export type Item = {
  id: string
  title: string
  description?: string
  /** UTC Date stringified on Y-M-D format */
  todoAt?: string
  /** UTC Date stringified on `RFC 3339` format  */
  updatedAt: string
  /** UTC Date stringified on `RFC 3339` format  */
  createdAt: string
}
export type Output = {
  count: number
  items: Item[]
}

export class InputParser implements ParsableInput<Input> {
  constructor(private readonly input: unknown) {}

  parse(): Either<ParseError<Input>, Input> {
    if (typeof this.input === 'object' && !!this.input) {
      return right(this.input)
    }
    return left({})
  }
}

export class OutputUtils {
  static createItemFromTodo(todo: Todo): Item {
    return {
      id: todo.id,
      title: todo.title,
      description: todo.description,
      todoAt: todo.todoAt ? DateUtils.utcYMD(todo.todoAt) : undefined,
      createdAt: DateUtils.utcRFC3339(todo.createdAt),
      updatedAt: DateUtils.utcRFC3339(todo.updatedAt),
    }
  }
}
