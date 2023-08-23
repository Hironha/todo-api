import { type InternalError } from '@core/helpers/error'

export const general: InternalError = {
  code: 'DTD-001',
  message: 'Internal server error',
  shortMessage: 'InternalError',
}

export const notFound: InternalError = {
  code: 'DTD-002',
  message: 'Todo not found',
  shortMessage: 'NotFound',
}
