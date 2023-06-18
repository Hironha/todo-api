type UnsetDetails = undefined | null | void | never

export type RequestError<D = null> = D extends UnsetDetails
  ? {
      code: string
      message: string
      shortMessage: string
    }
  : {
      code: string
      message: string
      shortMessage: string
      details: D
    }
