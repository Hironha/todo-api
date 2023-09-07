import { type RequestHandler } from 'express'

import * as E from '@core/helpers/either'
import { type ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { RemoveController, type RunError } from '@adapters/controllers/todo/remove'
import { type Input, InputParser } from '@adapters/dtos/todo/remove'
import { type ApiError } from '@framework/presentation/errors'

type HandlerFn = RequestHandler<{ id: string }, Record<PropertyKey, any>, never>

export class RemoveHandler {
  constructor(private repository: TodoRepository) {}

  handle: HandlerFn = async (req, res) => {
    const input = new InputParser(req.params)
    const controller = new RemoveController(input, this.repository)

    const output = await controller.run()
    if (E.isLeft(output)) {
      console.error(`REMOVE TODO ERROR: ${output.value}`)
      const [status, error] = this.getErrorResponseConfig(output.value)
      return res.status(status).json(error).end()
    }

    return res.status(204).end()
  }

  private getErrorResponseConfig(
    error: RunError
  ): [number, ApiError<ParseError<Input>['details']>] {
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
}
