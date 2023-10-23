pub mod todo;
pub mod tag;

pub trait Parse<T, E> {
    fn parse(self) -> Result<T, E>;
}
