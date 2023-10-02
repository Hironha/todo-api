pub mod todo;
pub mod tag;

pub trait ParsableInput<T, E> {
    fn parse(self) -> Result<T, E>;
}
