import { z } from 'zod'

import { useZodParser } from '@adapters/parser'

const createInputSchema = z.object({
  title: z.string(),
  description: z.string(),
  todoAt: z.coerce.date().optional(),
})

const getInputSchema = z.object({
  id: z.string(),
})

export const parseCreateInput = useZodParser(createInputSchema)

export const parseGetInput = useZodParser(getInputSchema)
