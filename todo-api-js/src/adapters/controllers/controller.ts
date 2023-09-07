import { type ParsableInput } from '@core/helpers/parser'

export abstract class AbstractController<I extends {}, O> {
  constructor(protected input: ParsableInput<I>) {}

  abstract run(): Promise<O>
}
