export type ApiError<D = never> = {
  code: string
  message: string
  details?: D
}
