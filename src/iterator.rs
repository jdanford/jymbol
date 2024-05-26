use std::result::Result;

// TODO: use generic implementation once stabilized
#[allow(clippy::module_name_repetitions)]
pub trait ResultIterator<T, E>: Iterator<Item = Result<T, E>> + Sized {
    fn try_collect(self) -> Result<Vec<T>, E> {
        self.collect()
    }
}

impl<T, E, I> ResultIterator<T, E> for I where I: Iterator<Item = Result<T, E>> {}
