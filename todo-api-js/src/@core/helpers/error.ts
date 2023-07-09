type UnsetDetails = undefined | null | void | never

type BaseInternalError = {
  code: string
  message: string
  shortMessage: string
}

export type InternalError<D = null> = D extends UnsetDetails
  ? BaseInternalError
  : BaseInternalError & {
      details: D
    }
