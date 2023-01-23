use crate::{symbol, Symbol, Token};

#[allow(clippy::module_name_repetitions)]
pub fn quote_type(t: &Token) -> Option<Symbol> {
    match t {
        Token::Quote => Some(*symbol::QUOTE),
        Token::Quasiquote => Some(*symbol::QUASIQUOTE),
        Token::Unquote => Some(*symbol::UNQUOTE),
        Token::UnquoteSplicing => Some(*symbol::UNQUOTE_SPLICING),
        _ => None,
    }
}
