import { type ApiError } from '@core/helpers/error'

export const general: ApiError = {
  code: 'GTD-001',
  message: 'Internal server error',
}

export const notFound: ApiError = {
  code: 'GTD-002',
  message: 'Todo not found',
}
