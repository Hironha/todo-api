import { z } from 'zod'

import { useZodParser } from '@adapters/parser'

const removeInputSchema = z.object({
  todoId: z.string(),
})

export const parseRemoveInput = useZodParser(removeInputSchema)
