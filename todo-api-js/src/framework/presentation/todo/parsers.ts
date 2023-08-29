import { z } from 'zod'

import { useZodParser } from '@adapters/parser'

const getInputSchema = z.object({
  todoId: z.string(),
})

const removeInputSchema = z.object({
  todoId: z.string(),
})

export const parseGetInput = useZodParser(getInputSchema)

export const parseRemoveInput = useZodParser(removeInputSchema)
