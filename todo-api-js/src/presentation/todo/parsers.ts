import { z } from 'zod'

import * as E from '@helpers/either'
import { type InternalError } from '@helpers/error'
import { type CreateInput } from '@functions/todo'

const createInputSchema = z.object({
  title: z.string(),
  description: z.string(),
  todoAt: z.coerce.date().optional(),
})

export const parseCreateInput = (input: unknown): E.Either<InternalError<unknown>, CreateInput> => {
  const validation = createInputSchema.safeParse(input)
  if (validation.success) {
    return E.right(validation.data)
  }
  return E.left({
    code: 'CTD-001',
    message: validation.error.message,
    shortMessage: 'ValidationError',
    details: validation.error.formErrors.fieldErrors,
  })
}
