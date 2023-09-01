import { z } from 'zod'

import * as E from '@core/helpers/either'
import { type DeserializationError, type ApiError } from '@core/helpers/error'
import { type View } from '@core/helpers/view'

export type Input = { id: string }

const removeInputSchema = z.object({
  id: z.string(),
})

export class InputView implements View<Input> {
  constructor(private value: Input) {}

  static parse(
    input: Record<PropertyKey, any>
  ): E.Either<ApiError<DeserializationError<Input>>, InputView> {
    const result = removeInputSchema.safeParse(input)
    if (result.success) {
      return E.right(new InputView(result.data))
    }

    const errors = result.error.flatten().fieldErrors
    const details: DeserializationError<Input> = {}

    if (errors.id) details.id = errors.id[0]

    return E.left({ code: 'VAL-001', message: 'validation error', details })
  }

  view(): Input {
    return this.value
  }
}
