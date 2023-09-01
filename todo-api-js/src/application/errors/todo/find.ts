import { type ApiError } from '@core/helpers/error'

export enum FindError {
  NotFound = 'NotFound',
  Internal = 'Internal',
}

export class FindErrorUtils {
  static toApi(error: FindError): ApiError {
    switch (error) {
      case FindError.NotFound:
        return { code: 'FTD-001', message: 'Todo not found' }
      case FindError.Internal:
        return { code: 'FTD-002', message: 'Internal server error' }
      default:
        const _: never = error
        return _
    }
  }
}
