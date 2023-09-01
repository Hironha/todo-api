import { type ApiError } from '@core/helpers/error'

export enum CreateError {
  Internal,
}

export class CreateErrorUtils {
  static toApi(error: CreateError): ApiError {
    switch (error) {
      case CreateError.Internal:
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
