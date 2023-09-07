import { type RequestHandler } from 'express'

import * as E from '@core/helpers/either'
import { type ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { InputParser, type Input } from '@adapters/dtos/todo/create'
import { CreateController, type RunError } from '@adapters/controllers/todo/create'
import { type ApiError } from '@framework/presentation/errors'

type HandlerFn = RequestHandler<{}, Record<PropertyKey, any>, Record<string, string>>

export class CreateHandler {
  constructor(private readonly repository: TodoRepository) {}

  handle: HandlerFn = async (req, res) => {
    const input = new InputParser(req.body)
    const controller = new CreateController(input, this.repository)
    const output = await controller.run()
    if (E.isLeft(output)) {
      console.error(`CREATE TODO ERROR: ${output.value}`)
      const [status, error] = this.getErrorResponseConfig(output.value)
      return res.status(status).json(error).end()
    }

    res.status(201).json(output.value).end()
  }

  private getErrorResponseConfig(
    error: RunError
  ): [number, ApiError<ParseError<Input>['details']>] {
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
}
