use logos::Span;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Error {
    pub message: String,
    pub line: usize,
    pub span: Span,
}

pub type Result<T> = std::result::Result<T, Error>;
