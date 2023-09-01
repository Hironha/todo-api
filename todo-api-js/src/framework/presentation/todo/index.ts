import { Router } from 'express'
import * as E from '@core/helpers/either'

import { list } from '@application/functions/todo/list'
import { remove } from '@application/functions/todo/remove'
import { TodoStore } from '@framework/store/todo'
import * as DeleteErrors from '@application/errors/todo/remove'

import { CreateController } from '@adapters/controllers/todo/create'
import { InputView as CreateInputView } from '@adapters/dtos/todo/create'

import { FindController } from '@adapters/controllers/todo/find'
import { InputView as FindInputView } from '@adapters/dtos/todo/find'

import { parseRemoveInput } from './parsers'

const router = Router({ strict: true })

const store = new TodoStore()

router.get<'/', never>('/', async (req, res) => {
  const output = await list({ repository: store })
  if (E.isLeft(output)) {
    return res.status(500).json(output.value).end()
  }
  return res.status(200).json(output.value).end()
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
    let status = output.value.kind === 'not-found' ? 404 : 500
    return res.status(status).json(output.value).end()
  }

  return res.status(200).json(output.value.view()).end()
})

router.delete<'/:todoId', { todoId: string }>('/:todoId', async (req, res) => {
  const input = parseRemoveInput(req.params)
  if (E.isLeft(input)) {
    return res.status(400).json(input.value).end()
  }

  const output = await remove({ repository: store, input: { id: input.value.todoId } })
  if (E.isLeft(output)) {
    const status = output.value.code === DeleteErrors.notFound.code ? 404 : 500
    return res.status(status).json(output.value).end()
  }

  return res.status(200).json(output.value).end()
})

export { router }
