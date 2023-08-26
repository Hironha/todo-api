import { type InternalError } from '@core/helpers/error'

export const general: InternalError = {
  code: 'GTD-001',
  message: 'Internal server error',
}

export const notFound: InternalError = {
  code: 'GTD-002',
  message: 'Todo not found',
}
