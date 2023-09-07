import { type RequestHandler } from 'express'

import * as E from '@core/helpers/either'
import { type ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { ListController, type RunError } from '@adapters/controllers/todo/list'
import { type Input, InputParser } from '@adapters/dtos/todo/list'
import { type ApiError } from '@framework/presentation/errors'

type HandlerFn = RequestHandler<never, Record<PropertyKey, any>, never>

export class ListHandler {
  constructor(private repository: TodoRepository) {}

  handle: HandlerFn = async (_, res) => {
    const input = new InputParser({})
    const controller = new ListController(input, this.repository)
    const output = await controller.run()
    if (E.isLeft(output)) {
      const [status, error] = this.getErrorResponseConfig(output.value)
      return res.status(status).json(error).end()
    }

    return res.status(200).json(output.value)
  }

  private getErrorResponseConfig(
    error: RunError
  ): [number, ApiError<ParseError<Input>['details']>] {
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
}
