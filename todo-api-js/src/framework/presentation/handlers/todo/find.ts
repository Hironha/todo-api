import { type Context } from 'elysia'

import * as E from '@core/helpers/either'
import { ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { FindController, type RunError } from '@adapters/controllers/todo/find'
import { RawInput, type Input, type Output } from '@adapters/dtos/todo/find'
import { type ApiError } from '@framework/presentation/errors'

export type FindHandlerState = { repository: TodoRepository }
export type FindHandlerOutput = Output | ApiError<ParseError<Input>['details']>
export type FindHandlerContext = Context<
  {
    params: { id: string }
    response: FindHandlerOutput
  },
  FindHandlerState
>

export async function findHandler(context: FindHandlerContext): Promise<FindHandlerOutput> {
  const input = new RawInput(context.params)
  const controller = new FindController(input, context.store.repository)

  const output = await controller.run()
  if (E.isLeft(output)) {
    console.error(`FIND TODO ERROR: ${output.value}`)
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
      return [400, { code: 'FTD-001', message: 'Invalid input', details: error.details }]
    case 'not-found':
      return [404, { code: 'FTD-002', message: 'Todo not found' }]
    case 'internal':
      return [500, { code: 'FTD-003', message: 'Internal server error' }]
    default:
      // exhaustive check
      return error
  }
}
