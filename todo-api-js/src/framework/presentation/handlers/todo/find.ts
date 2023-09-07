import { type RequestHandler } from 'express'

import * as E from '@core/helpers/either'
import { ParseError } from '@core/helpers/parser'
import { type TodoRepository } from '@application/repositories/todo'
import { FindController, type RunError } from '@adapters/controllers/todo/find'
import { type Input, InputParser } from '@adapters/dtos/todo/find'
import { type ApiError } from '@framework/presentation/errors'

type HandlerFn = RequestHandler<{ id: string }, Record<PropertyKey, any>, never>

export class FindHandler {
  constructor(private readonly repository: TodoRepository) {}

  handle: HandlerFn = async (req, res) => {
    const input = new InputParser(req.params)
    const controller = new FindController(input, this.repository)

    const output = await controller.run()
    if (E.isLeft(output)) {
      console.error(`FIND TODO ERROR: ${output.value}`)
      const [status, error] = this.getErrorResponseConfig(output.value)
      return res.status(status).json(error).end()
    }

    return res.status(200).json(output.value).end()
  }

  private getErrorResponseConfig(
    error: RunError
  ): [number, ApiError<ParseError<Input>['details']>] {
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
}
