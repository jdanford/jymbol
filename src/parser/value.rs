use chumsky::prelude::*;

use crate::{
    parser::primitive::{float, string},
    Value,
};

static NON_SYMBOL_CHARS: &str = "()[]{}\"'`,@.";

fn is_symbol(c: char) -> bool {
    !c.is_control() && !c.is_whitespace() && !NON_SYMBOL_CHARS.contains(c)
}

fn is_symbol_head(c: char) -> bool {
    is_symbol(c) && !c.is_ascii_digit()
}

fn raw_symbol() -> impl Parser<char, String, Error = Simple<char>> {
    let symbol_head = filter(|&c| is_symbol_head(c));
    let symbol_tail = filter(|&c| is_symbol(c)).repeated();
    symbol_head.chain(symbol_tail).collect()
}

fn raw_expr() -> impl Parser<char, Value, Error = Simple<char>> {
    recursive(|expr| {
        let symbol = raw_symbol().map(Value::symbol).labelled("symbol");
        let number = float().map(Value::from).labelled("number");
        let string = string().map(Value::from).labelled("string");

        let quote = just('\'')
            .ignore_then(expr.clone())
            .map(Value::quote)
            .labelled("quote");

        let quasiquote = just('`')
            .ignore_then(expr.clone())
            .map(Value::quasiquote)
            .labelled("quasiquote");

        let unquote = just(',')
            .ignore_then(expr.clone())
            .map(Value::unquote)
            .labelled("unquote");

        let unquote_splicing = just(",@")
            .ignore_then(expr.clone())
            .map(Value::unquote_splicing)
            .labelled("unquote_splicing");

        let list = expr
            .clone()
            .repeated()
            .delimited_by(just('('), just(')'))
            .map(Value::list)
            .labelled("list");

        let square_list = expr
            .clone()
            .repeated()
            .delimited_by(just('['), just(']'))
            .map(Value::list)
            .labelled("square_list");

        choice((
            number,
            string,
            quote,
            quasiquote,
            unquote,
            unquote_splicing,
            list,
            square_list,
            symbol,
        ))
        .padded()
    })
}

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> {
    raw_expr().then_ignore(end())
}

#[allow(dead_code)]
pub fn values() -> impl Parser<char, Vec<Value>, Error = Simple<char>> {
    raw_expr().repeated().then_ignore(end())
}
