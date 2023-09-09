import { type Context } from 'elysia'

import * as E from '@core/helpers/either'
import { type ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { RawInput, type Input, type Output } from '@adapters/dtos/todo/create'
import { CreateController, type RunError } from '@adapters/controllers/todo/create'
import { type ApiError } from '@framework/presentation/errors'

export type CreateHandlerState = { repository: TodoRepository }
export type CreateHandlerOutput = Output | ApiError<ParseError<Input>['details']>
export type CreateHandlerContext = Context<
  {
    response: Promise<CreateHandlerOutput>
  },
  CreateHandlerState
>

export async function createHandler(context: CreateHandlerContext): Promise<CreateHandlerOutput> {
  const input = new RawInput(context.body)
  const controller = new CreateController(input, context.store.repository)
  const output = await controller.run()
  if (E.isLeft(output)) {
    console.error(`CREATE TODO ERROR: ${JSON.stringify(output.value)}`)
    const [status, error] = getErrorResponseConfig(output.value)
    context.set.status = status
    return error
  }

  context.set.status = 201
  return output.value
}

function getErrorResponseConfig(error: RunError): [number, ApiError<ParseError<Input>['details']>] {
  switch (error.kind) {
    case 'validation':
      return [400, { code: 'CTD-001', message: 'Invalid input', details: error.details }]
    case 'internal':
      return [500, { code: 'CTD-002', message: 'Internal server error' }]
    default:
      // exhaustive check
      return error
  }
}
