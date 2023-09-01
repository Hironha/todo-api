import { DateUtils } from '@core/helpers/date'
import { type View } from '@core/helpers/view'
import { type Todo } from '@domain/entities/todo'

export type Item = {
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

export type Output = {
  count: number
  items: Item[]
}

export class OutputView implements View<Output> {
  private items: Item[] = []
  private count: number = 0

  setItemsFromTodos(todos: Todo[]): this {
    this.items = todos.map(t => ({
      id: t.id,
      title: t.title,
      description: t.description,
      todoAt: t.todoAt ? DateUtils.utcYMD(t.todoAt) : undefined,
      createdAt: DateUtils.utcRFC3339(t.createdAt),
      updatedAt: DateUtils.utcRFC3339(t.updatedAt),
    }))

    return this
  }

  setCount(count: number): this {
    this.count = count
    return this
  }

  view(): Output {
    return { count: this.count, items: this.items }
  }
}
