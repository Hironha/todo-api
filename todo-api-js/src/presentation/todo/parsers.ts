import { z } from 'zod'

import { useZodParser } from '@adapters/parser'

const createInputSchema = z.object({
  title: z.string({ required_error: 'Field title is required' }),
  description: z.string({ required_error: 'Field description is required' }),
  todoAt: z.coerce.date().optional(),
})

const getInputSchema = z.object({
  todoId: z.string(),
})

const deleteInputSchema = z.object({
  todoId: z.string(),
})

export const parseCreateInput = useZodParser(createInputSchema)

export const parseGetInput = useZodParser(getInputSchema)

export const parseDeleteInput = useZodParser(deleteInputSchema)
