pub mod todo;

pub trait ParsableInput<T, E> {
    fn parse(self) -> Result<T, E>;
}
