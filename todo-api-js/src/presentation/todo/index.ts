import { Router } from 'express'
import * as E from '@helpers/either'

import { create } from '@functions/todo'
import { parseCreateInput } from './parsers'

const router = Router({ strict: true })

router.post<'/', never>('/', async (req, res) => {
  const input = parseCreateInput(req.body)
  if (E.isLeft(input)) {
    return res.status(400).json(input.value).end()
  }

  const output = await create({ repository: {} as any, input: input.value })
  if (E.isLeft(output)) {
    return res.status(500).json(output.value).end()
  }

  res.status(201).json(output.value).end()
})

export { router }
