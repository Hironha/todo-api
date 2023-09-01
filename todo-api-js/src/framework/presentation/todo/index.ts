import { Router } from 'express'
import * as E from '@core/helpers/either'

import { TodoStore } from '@framework/store/todo'

import { CreateController } from '@adapters/controllers/todo/create'
import { InputView as CreateInputView } from '@adapters/dtos/todo/create'

import { FindController } from '@adapters/controllers/todo/find'
import { InputView as FindInputView } from '@adapters/dtos/todo/find'

import { RemoveController } from '@adapters/controllers/todo/remove'
import { InputView as RemoveInputView } from '@adapters/dtos/todo/remove'

import { ListController } from '@adapters/controllers/todo/list'

const router = Router({ strict: true })

const store = new TodoStore()

router.get<'/', never>('/', async (req, res) => {
  const output = await new ListController(store).run()
  if (E.isLeft(output)) {
    return res.status(500).json(output.value).end()
  }

  return res.status(200).json(output.value.view()).end()
})

router.post<'/', never>('/', async (req, res) => {
  const input = CreateInputView.parse(req.body)
  if (E.isLeft(input)) {
    return res.status(400).json(input.value).end()
  }

  const controller = new CreateController(store)
  const output = await controller.run(input.value)
  if (E.isLeft(output)) {
    return res.status(500).json(output.value.error).end()
  }

  res.status(201).json(output.value.view()).end()
})

router.get<'/:todoId', { todoId: string }>('/:todoId', async (req, res) => {
  const input = FindInputView.parse(req.params)
  if (E.isLeft(input)) {
    return res.status(400).json(input.value).end()
  }

  const controller = new FindController(store)
  const output = await controller.run(input.value)
  if (E.isLeft(output)) {
    const status = output.value.kind === 'not-found' ? 404 : 500
    return res.status(status).json(output.value).end()
  }

  return res.status(200).json(output.value.view()).end()
})

router.delete<'/:todoId', { todoId: string }>('/:todoId', async (req, res) => {
  const input = RemoveInputView.parse(req.params)
  if (E.isLeft(input)) {
    return res.status(400).json(input.value).end()
  }

  const controller = new RemoveController(store)
  const output = await controller.run(input.value)
  if (E.isLeft(output)) {
    const status = output.value.kind === 'not-found' ? 404 : 500
    return res.status(status).json(output.value).end()
  }

  return res.status(204).json(output.value).end()
})

export { router }
