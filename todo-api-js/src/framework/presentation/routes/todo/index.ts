import { Elysia } from 'elysia'

import { createHandler } from '@framework/presentation/handlers/todo/create'
import { findHandler } from '@framework/presentation/handlers/todo/find'
import { removeHandler } from '@framework/presentation/handlers/todo/remove'
import { listHandler } from '@framework/presentation/handlers/todo/list'
import { TodoStore } from '@framework/store/todo'

export function createTodoRouter(path: string) {
  const store = new TodoStore()

  return new Elysia({ prefix: path, strictPath: true })
    .state('repository', store)
    .get('/', listHandler)
    .post('/', createHandler)
    .get('/:id', findHandler)
    .delete('/:id', removeHandler)
}
