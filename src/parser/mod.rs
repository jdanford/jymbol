mod primitive;
mod value;

use chumsky::{prelude::Simple, Parser};
pub use value::value;

use crate::Result;

#[must_use]
pub fn collect_errors(errors: Vec<Simple<char>>) -> String {
    let error_strings: Vec<String> = errors.into_iter().map(|err| err.to_string()).collect();
    error_strings.join("\n")
}

pub fn parse<T, S: AsRef<str>>(
    s: S,
    parser: impl Parser<char, T, Error = Simple<char>>,
) -> Result<T> {
    parser.parse(s.as_ref()).map_err(collect_errors)
}
