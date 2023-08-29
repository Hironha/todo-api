import { z } from 'zod'
import { useZodParser } from '@adapters/parser'

const createInputSchema = z.object({
  title: z.string({ required_error: 'title is required' }),
  description: z.string({ required_error: 'description is required' }),
  todoAt: z.coerce.date().optional(),
})

export const parser = useZodParser(createInputSchema)
