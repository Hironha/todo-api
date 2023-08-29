import { type ApiError } from '@core/helpers/error'

export const general: ApiError = {
  code: 'DTD-001',
  message: 'Internal server error',
}

export const notFound: ApiError = {
  code: 'DTD-002',
  message: 'Todo not found',
}
