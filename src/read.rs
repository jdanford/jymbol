use chumsky::{error::Simple, Parser};

use crate::{
    parser::{self, Expr},
    symbol, Result, ResultIterator, Value,
};

pub fn expr<S: AsRef<str>>(input: S) -> Result<Value> {
    let parser = parser::expr();
    let expr = parser
        .parse(input.as_ref())
        .map_err(collect_parser_errors)?;
    reify(expr)
}

pub fn exprs<S: AsRef<str>>(input: S) -> Result<Vec<Value>> {
    let parser = parser::exprs();
    let exprs = parser
        .parse(input.as_ref())
        .map_err(collect_parser_errors)?;
    exprs.into_iter().map(reify).try_collect()
}

fn reify(expr: Expr) -> Result<Value> {
    match expr {
        Expr::Number(num) => Ok(Value::Number(num)),
        Expr::Symbol(sym) => Ok(sym.into()),
        Expr::String(s) => Ok(s.into()),
        Expr::Quote(expr) => {
            let value = reify(*expr)?;
            Value::compound(*symbol::QUOTE, vec![value])
        }
        Expr::Quasiquote(expr) => {
            let value = reify(*expr)?;
            Value::compound(*symbol::QUASIQUOTE, vec![value])
        }
        Expr::Unquote(expr) => {
            let value = reify(*expr)?;
            Value::compound(*symbol::UNQUOTE, vec![value])
        }
        Expr::UnquoteSplicing(expr) => {
            let value = reify(*expr)?;
            Value::compound(*symbol::UNQUOTE_SPLICING, vec![value])
        }
        Expr::List(exprs) => {
            let values = exprs.into_iter().map(reify).try_collect()?;
            Value::list(&values)
        }
    }
}

fn collect_parser_errors(errors: Vec<Simple<char>>) -> String {
    let error_strings: Vec<String> = errors.into_iter().map(|err| err.to_string()).collect();
    error_strings.join("\n")
}
