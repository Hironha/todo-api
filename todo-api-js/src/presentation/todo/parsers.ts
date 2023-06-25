import { z } from 'zod'

import * as E from '@helpers/either'
import { RequestError } from '@helpers/error'
import { type CreateInput } from '@functions/todo'

const createInputSchema = z.object({
  title: z.string(),
  description: z.string(),
  todoAt: z.coerce.date().optional(),
})

export const parseCreateInput = (input: unknown): E.Either<RequestError<unknown>, CreateInput> => {
  const validation = createInputSchema.safeParse(input)
  if (validation.success) {
    return E.right(validation.data)
  }
  return E.right({
    code: 'CTD-001',
    message: validation.error.message,
    shortMessage: 'ValidationError',
    details: validation.error.formErrors.fieldErrors,
  })
}
