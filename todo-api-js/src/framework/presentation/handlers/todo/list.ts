import { type RequestHandler } from 'express'

import * as E from '@core/helpers/either'
import { type TodoRepository } from '@application/repositories/todo'
import { ListController, type RunError } from '@adapters/controllers/todo/list'
import { type ApiError } from '@framework/presentation/errors'

type HandlerFn = RequestHandler<never, Record<PropertyKey, any>, never>

export class ListHandler {
  private controller: ListController

  constructor(repository: TodoRepository) {
    this.controller = new ListController(repository)
  }

  handle: HandlerFn = async (_, res) => {
    const output = await this.controller.run()
    if (E.isLeft(output)) {
      const [status, error] = this.getErrorResponseConfig(output.value)
      return res.status(status).json(error).end()
    }

    return res.status(200).json(output.value.view())
  }

  private getErrorResponseConfig(error: RunError): [number, ApiError<unknown>] {
    switch (error.kind) {
      case 'internal':
        return [500, { code: 'LTD-001', message: 'Internal server error' }]
      default:
        return error.kind
    }
  }
}
