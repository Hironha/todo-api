import { Router } from 'express'
import { type TodoRepository } from '@application/repositories/todo'
import { CreateHandler } from '@framework/presentation/handlers/todo/create'
import { FindHandler } from '@framework/presentation/handlers/todo/find'
import { RemoveHandler } from '@framework/presentation/handlers/todo/remove'
import { ListHandler } from '@framework/presentation/handlers/todo/list'

export function createTodoRouter(repository: TodoRepository): Router {
  const router = Router({ strict: true, caseSensitive: true })

  router.get('/', new ListHandler(repository).handle)
  router.get('/:id', new FindHandler(repository).handle)
  router.post('/', new CreateHandler(repository).handle)
  router.delete('/:id', new RemoveHandler(repository).handle)

  return router
}
