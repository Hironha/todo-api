import { type RequestHandler } from 'express'

import * as E from '@core/helpers/either'
import { type TodoRepository } from '@application/repositories/todo'
import { RemoveController, type RunError } from '@adapters/controllers/todo/remove'
import { InputView } from '@adapters/dtos/todo/remove'
import { type ApiError } from '@framework/presentation/errors'
import { createParseError } from '@framework/presentation/errors'

type HandlerFn = RequestHandler<{ id: string }, Record<PropertyKey, any>, never>

export class RemoveHandler {
  private controller: RemoveController

  constructor(repository: TodoRepository) {
    this.controller = new RemoveController(repository)
  }

  handle: HandlerFn = async (req, res) => {
    const input = InputView.parse(req.params)
    if (E.isLeft(input)) {
      const error = createParseError(input.value)
      return res.status(400).json(error).end()
    }

    const output = await this.controller.run(input.value)
    if (E.isLeft(output)) {
      console.error(`REMOVE TODO ERROR: ${output.value}`)
      const [status, error] = this.getErrorResponseConfig(output.value)
      return res.status(status).json(error).end()
    }

    return res.status(204).end()
  }

  private getErrorResponseConfig(error: RunError): [number, ApiError] {
    switch (error.kind) {
      case 'not-found':
        return [404, { code: 'FTD-001', message: 'Todo not found' }]
      case 'internal':
        return [500, { code: 'FTD-002', message: 'Internal server error' }]
      default:
        // exhaustive check
        return error
    }
  }
}
