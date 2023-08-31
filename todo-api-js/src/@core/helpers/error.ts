export type DeserializationError<From extends {}> = {
  [P in keyof From]?: string
}

export type ApiError<D = never> = {
  code: string
  message: string
  details?: D
}
