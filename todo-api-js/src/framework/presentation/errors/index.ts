export type BasicError = {
  code: string
  message: string
}

export type DetailedError<Details extends {}> = BasicError & {
  details: Details
}

export type UnsetDetails = undefined | void | null

export type ApiError<D extends {}> = {
  code: string
  message: string
  details?: D
}
