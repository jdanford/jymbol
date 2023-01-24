use chumsky::prelude::*;

use crate::Symbol;

pub enum Expr {
    Number(f64),
    Symbol(Symbol),
    Quote(Box<Expr>),
    Quasiquote(Box<Expr>),
    Unquote(Box<Expr>),
    UnquoteSplicing(Box<Expr>),
    List(Vec<Expr>),
}

const NON_SYMBOL_CHARS: &str = "()[]{}\"'`,@";

fn is_symbol(c: char) -> bool {
    !c.is_control() && !c.is_whitespace() && !NON_SYMBOL_CHARS.contains(c)
}

fn is_symbol_head(c: char) -> bool {
    is_symbol(c) && !c.is_ascii_digit() && c != '-'
}

#[allow(clippy::needless_pass_by_value)]
fn expr_number(s: String) -> Expr {
    let f = s.parse().expect("invalid number");
    Expr::Number(f)
}

fn expr_symbol(s: String) -> Expr {
    Expr::Symbol(Symbol::new(s))
}

fn expr_quote(expr: Expr) -> Expr {
    Expr::Quote(Box::new(expr))
}

fn expr_quasiquote(expr: Expr) -> Expr {
    Expr::Quasiquote(Box::new(expr))
}

fn expr_unquote(expr: Expr) -> Expr {
    Expr::Unquote(Box::new(expr))
}

fn expr_unquote_splicing(expr: Expr) -> Expr {
    Expr::UnquoteSplicing(Box::new(expr))
}

fn expr_list(exprs: Vec<Expr>) -> Expr {
    Expr::List(exprs)
}

pub fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let frac = just('.').chain(text::digits(10));

        let exp = just('e')
            .or(just('E'))
            .chain(just('+').or(just('-')).or_not())
            .chain::<char, _, _>(text::digits(10));

        let number = just('-')
            .or_not()
            .chain::<char, _, _>(text::int(10))
            .chain::<char, _, _>(frac.or_not().flatten())
            .chain::<char, _, _>(exp.or_not().flatten())
            .collect::<String>()
            .map(expr_number)
            .labelled("number");

        // let escape = just::<char, char, Simple<char>>('\\').ignore_then(
        //     just('\\')
        //         .or(just('/'))
        //         .or(just('"'))
        //         .or(just('b').to('\x08'))
        //         .or(just('f').to('\x0C'))
        //         .or(just('n').to('\n'))
        //         .or(just('r').to('\r'))
        //         .or(just('t').to('\t'))
        //         .or(just('u').ignore_then(
        //             filter(char::is_ascii_hexdigit)
        //                 .repeated()
        //                 .exactly(4)
        //                 .collect::<String>()
        //                 .validate(|digits, span, emit| {
        //                     char::from_u32(u32::from_str_radix(&digits, 16).unwrap())
        //                         .unwrap_or_else(|| {
        //                             emit(Simple::custom(span, "invalid unicode character"));
        //                             '\u{FFFD}' // unicode replacement character
        //                         })
        //                 }),
        //         )),
        // );

        // let string = just('"')
        //     .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
        //     .then_ignore(just('"'))
        //     .collect::<String>()
        //     .labelled("string");

        let symbol_head = filter(|c| is_symbol_head(*c));
        let symbol_tail = filter(|c| is_symbol(*c)).repeated();
        let symbol = symbol_head
            .chain(symbol_tail)
            .collect()
            .map(expr_symbol)
            .labelled("symbol");

        let quote = just('\'')
            .ignore_then(expr.clone())
            .map(expr_quote)
            .labelled("quote");

        let quasiquote = just('`')
            .ignore_then(expr.clone())
            .map(expr_quasiquote)
            .labelled("quasiquote");

        let unquote = just(',')
            .ignore_then(expr.clone())
            .map(expr_unquote)
            .labelled("unquote");

        let unquote_splicing = just(",@")
            .ignore_then(expr.clone())
            .map(expr_unquote_splicing)
            .labelled("unquote-splicing");

        let list = expr
            .clone()
            .repeated()
            .delimited_by(just('('), just(')'))
            .map(expr_list)
            .labelled("list");

        choice((
            number,
            symbol,
            quote,
            quasiquote,
            unquote,
            unquote_splicing,
            list,
        ))
        .padded()
    })
    .then_ignore(end())
}
