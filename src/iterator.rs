use std::result;

// TODO: use generic implementation once stabilized
#[allow(clippy::module_name_repetitions)]
pub trait ResultIterator<T, E>: Iterator<Item = result::Result<T, E>> + Sized {
    fn try_collect(self) -> result::Result<Vec<T>, E> {
        self.collect()
    }
}

impl<T, E, I: Iterator<Item = result::Result<T, E>>> ResultIterator<T, E> for I {}
