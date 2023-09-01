export interface Controller<I, R> {
  run(input: I): Promise<R>
}
