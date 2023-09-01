import { type ApiError } from '@core/helpers/error'

export enum ListError {
  Internal = 'Internal',
}

export class ListErrorUtils {
  static toApi(error: ListError): ApiError {
    switch (error) {
      case ListError.Internal:
        return { code: 'LTD-001', message: 'Internal server error' }
      default:
        const _: never = error
        return error
    }
  }
}
