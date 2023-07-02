import { Router } from 'express'
import * as E from '@helpers/either'

import { create } from '@functions/todo/create'
import { get } from '@functions/todo/get'
import { parseCreateInput, parseGetInput } from './parsers'
import { TodoStore } from '@store/todo'

const router = Router({ strict: true })

const store = new TodoStore()

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

  const output = await get({ repository: store, input: input.value })
  if (E.isLeft(output)) {
    return res.status(500).json(output.value).end()
  }

  return res.status(200).json(output.value).end()
})

export { router }
