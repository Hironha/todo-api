import { type Context } from 'elysia'

import * as E from '@core/helpers/either'
import { type ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { ListController, type RunError } from '@adapters/controllers/todo/list'
import { RawInput, type Input, type Output } from '@adapters/dtos/todo/list'
import { type ApiError } from '@framework/presentation/errors'

export type ListHandlerState = { repository: TodoRepository }
export type ListHandlerOutput = Output | ApiError<ParseError<Input>['details']>
export type ListHandlerContext = Context<
  {
    response: Promise<ListHandlerOutput>
  },
  ListHandlerState
>

export async function listHandler(context: ListHandlerContext): Promise<ListHandlerOutput> {
  const input = new RawInput({})
  const controller = new ListController(input, context.store.repository)
  const output = await controller.run()
  if (E.isLeft(output)) {
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
      return [400, { code: 'LTD-001', message: 'Invalid input', details: error.details }]
    case 'internal':
      return [500, { code: 'LTD-002', message: 'Internal server error' }]
    default:
      // exhaustive check
      return error
  }
}
