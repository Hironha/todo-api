pub trait UseCase<I, O> {
    async fn exec(self, input: I) -> O;
}
