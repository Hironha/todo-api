import { type Either, right, left } from '@core/helpers/either'
import { type ParseError, type ParsableInput } from '@core/helpers/parser'
import { type Todo } from '@domain/entities/todo'
import { TodoViewUtils, type TodoView } from '@adapters/views/todo'

export type Input = {}
export type Item = TodoView & {}
export type Output = {
  count: number
  items: Item[]
}

export class RawInput implements ParsableInput<Input> {
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
    return TodoViewUtils.fromTodo(todo)
  }
}
