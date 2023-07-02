import { Router } from 'express'
import * as E from '@helpers/either'

import { create } from '@functions/todo'
import { parseCreateInput } from './parsers'
import { TodoStore } from '@store/todo'

const router = Router({ strict: true })

const store = new TodoStore()

router.post<'/', never>('/', async (req, res) => {
  const input = parseCreateInput(req.body)
  if (E.isLeft(input)) {
    return res.status(input.value.status).json(input.value).end()
  }

  const output = await create({ repository: store, input: input.value })
  if (E.isLeft(output)) {
    return res.status(output.value.status).json(output.value).end()
  }

  res.status(201).json(output.value).end()
})

export { router }
