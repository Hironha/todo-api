import { type RequestHandler } from 'express'

import * as E from '@core/helpers/either'
import { type TodoRepository } from '@application/repositories/todo'
import { InputView } from '@adapters/dtos/todo/create'
import { CreateController, type RunError } from '@adapters/controllers/todo/create'
import { type ApiError, createParseError } from '@framework/presentation/errors'

type HandlerFn = RequestHandler<{}, Record<PropertyKey, any>, Record<string, string>>

export class CreateHandler {
  private controller: CreateController

  constructor(repository: TodoRepository) {
    this.controller = new CreateController(repository)
  }

  handle: HandlerFn = async (req, res) => {
    const input = InputView.parse(req.body)
    if (E.isLeft(input)) {
      const error = createParseError(input.value)
      return res.status(400).json(error).end()
    }

    const output = await this.controller.run(input.value)
    if (E.isLeft(output)) {
      console.error(`CREATE TODO ERROR: ${output.value}`)
      const [status, error] = this.getErrorResponseConfig(output.value)
      return res.status(status).json(error).end()
    }

    res.status(201).json(output.value.view()).end()
  }

  private getErrorResponseConfig(error: RunError): [number, ApiError<unknown>] {
    switch (error.kind) {
      case 'internal':
        return [500, { code: 'CTD-001', message: 'Internal server error' }]
      default:
        // exhaustive check
        return error.kind
    }
  }
}
