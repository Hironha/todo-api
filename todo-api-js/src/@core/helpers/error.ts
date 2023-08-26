type UnsetDetails = undefined | null | void | never

type BaseInternalError = {
  code: string
  message: string
}

type DetailedInternalError<D> = {
  code: string
  message: string
  details: D
}

export type InternalError<D = null> = D extends UnsetDetails
  ? BaseInternalError
  : DetailedInternalError<D>
