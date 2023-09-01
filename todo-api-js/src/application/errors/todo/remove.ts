import { type ApiError } from '@core/helpers/error'

export enum RemoveError {
  NotFound,
  InternalError,
}

export class RemoveErrorUtils {
  static toApi(error: RemoveError): ApiError {
    switch (error) {
      case RemoveError.NotFound:
        return { code: 'DTD-002', message: 'Todo not found' }
      case RemoveError.InternalError:
        return { code: 'DTD-001', message: 'Internal server error' }
      default:
        const _: never = error
        return _
    }
  }
}
