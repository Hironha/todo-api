import { z } from 'zod'

import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'
import { type CreateInput } from '@functions/todo/create'
import { type GetInput } from '@functions/todo/get'

const createInputSchema = z.object({
  title: z.string(),
  description: z.string(),
  todoAt: z.coerce.date().optional(),
})

const getInputSchema = z.object({
  id: z.string(),
})

export const parseCreateInput = (input: unknown): E.Either<InternalError<unknown>, CreateInput> => {
  const validation = createInputSchema.safeParse(input)
  if (!validation.success) {
    return E.left({
      code: 'VAL-001',
      message: validation.error.message,
      shortMessage: 'ValidationError',
      details: validation.error.formErrors.fieldErrors,
    })
  }
  return E.right(validation.data)
}

export const parseGetInput = (input: unknown): E.Either<InternalError<unknown>, GetInput> => {
  const validation = getInputSchema.safeParse(input)
  if (!validation.success) {
    return E.left({
      code: 'VAL-001',
      message: validation.error.message,
      shortMessage: 'ValidationError',
      details: validation.error.formErrors.fieldErrors,
    })
  }
  return E.right(validation.data)
}
