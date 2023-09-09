import { type Context } from 'elysia'

import * as E from '@core/helpers/either'
import { type ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { RemoveController, type RunError } from '@adapters/controllers/todo/remove'
import { type Input, InputParser } from '@adapters/dtos/todo/remove'
import { type ApiError } from '@framework/presentation/errors'

export type RemoveHandlerState = { repository: TodoRepository }
export type RemoveHandlerOutput = void | ApiError<ParseError<Input>['details']>
export type RemoveHandlerContext = Context<
  {
    params: { id: string }
    response: RemoveHandlerOutput
  },
  RemoveHandlerState
>

export async function removeHandler(context: RemoveHandlerContext): Promise<RemoveHandlerOutput> {
  const input = new InputParser(context.params)
  const controller = new RemoveController(input, context.store.repository)

  const output = await controller.run()
  if (E.isLeft(output)) {
    console.error(`REMOVE TODO ERROR: ${output.value}`)
    const [status, error] = getErrorResponseConfig(output.value)
    context.set.status = status
    return error
  }

  context.set.status = 204
}

function getErrorResponseConfig(error: RunError): [number, ApiError<ParseError<Input>['details']>] {
  switch (error.kind) {
    case 'validation':
      return [400, { code: 'RTD-001', message: 'Invalid input', details: error.details }]
    case 'not-found':
      return [404, { code: 'RTD-002', message: 'Todo not found' }]
    case 'internal':
      return [500, { code: 'RTD-003', message: 'Internal server error' }]
    default:
      // exhaustive check
      return error
  }
}
