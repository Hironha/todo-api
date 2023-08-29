import { type ApiError } from '@core/helpers/error'

export enum CreateError {
  InternalError,
}

export class CreateErrorUtils {
  static toInternalError(error: CreateError): ApiError {
    switch (error) {
      case CreateError.InternalError:
        return {
          code: 'CTD-001',
          message: 'Internal server error',
        }
      default:
        const _: never = error
        return _
    }
  }
}
