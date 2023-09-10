import { type Context } from 'elysia'

import * as E from '@core/helpers/either'
import { type ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { RawInput, type Input, type Output } from '@adapters/dtos/todo/update'
import { UpdateController, type RunError } from '@adapters/controllers/todo/update'
import { type ApiError } from '@framework/presentation/errors'

export type UpdateHandlerState = { repository: TodoRepository }
export type UpdateHandlerOutput = Output | ApiError<ParseError<Input>['details']>
export type UpdateHandlerContext = Context<
  {
    params: { id: string }
    response: Promise<UpdateHandlerOutput>
  },
  UpdateHandlerState
>

export async function updateHandler(context: UpdateHandlerContext): Promise<UpdateHandlerOutput> {
  const input = new RawInput(context.body)
  const controller = new UpdateController(input, context.store.repository)
  const output = await controller.run()
  if (E.isLeft(output)) {
    console.error(`CREATE TODO ERROR: ${JSON.stringify(output.value)}`)
    const [status, error] = getErrorResponseConfig(output.value)
    context.set.status = status
    return error
  }

  context.set.status = 200
  return output.value
}

function getErrorResponseConfig(error: RunError): [number, ApiError<ParseError<Input>['details']>] {
  switch (error.kind) {
    case 'validation':
      return [400, { code: 'UTD-001', message: 'Invalid input', details: error.details }]
    case 'not-found':
      return [404, { code: 'UTD-002', message: 'Todo not found' }]
    case 'internal':
      return [500, { code: 'UTD-003', message: 'Internal server error' }]
    default:
      // exhaustive check
      return error
  }
}
