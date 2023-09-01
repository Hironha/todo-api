import { type ApiError } from '@core/helpers/error'

export enum FindError {
  NotFound = 'NotFound',
  InternalError = 'InternalError',
}

export class FindErrorUtils {
  static toInternalError(error: FindError): ApiError {
    switch (error) {
      case FindError.NotFound:
        return { code: 'FTD-001', message: 'Todo not found' }
      case FindError.InternalError:
        return { code: 'FTD-002', message: 'Internal server error' }
      default:
        const _: never = error
        return _
    }
  }
}
