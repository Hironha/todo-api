import { Router } from 'express'
import * as E from '@helpers/either'

import { get } from '@functions/todo/get'
import { create } from '@functions/todo/create'
import { list } from '@functions/todo/list'
import { remove } from '@functions/todo/remove'
import { TodoStore } from '@store/todo'
import * as GetErrors from '@errors/todo/get'
import * as DeleteErrors from '@errors/todo/remove'
import { parseCreateInput, parseGetInput, parseRemoveInput } from './parsers'

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
  const input = parseCreateInput(req.body)
  if (E.isLeft(input)) {
    return res.status(400).json(input.value).end()
  }

  const output = await create({ repository: store, input: input.value })
  if (E.isLeft(output)) {
    return res.status(500).json(output.value).end()
  }

  res.status(201).json(output.value).end()
})

router.get<'/:todoId', { todoId: string }>('/:todoId', async (req, res) => {
  const input = parseGetInput(req.params)
  if (E.isLeft(input)) {
    return res.status(400).json(input.value).end()
  }

  const output = await get({ repository: store, input: { id: input.value.todoId } })
  if (E.isLeft(output)) {
    const status = output.value.code === GetErrors.notFound.code ? 404 : 500
    return res.status(status).json(output.value).end()
  }

  return res.status(200).json(output.value).end()
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
