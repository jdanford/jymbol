use chumsky::{error::Simple, Parser};

use crate::{parser::Expr, symbol, Result, Value};

fn collect_parser_errors(errors: Vec<Simple<char>>) -> String {
    let error_strings: Vec<String> = errors.into_iter().map(|err| err.to_string()).collect();
    error_strings.join("\n")
}

fn parse<S: AsRef<str>>(input: S) -> Result<Expr> {
    let parser = crate::parser::parser();
    parser.parse(input.as_ref()).map_err(collect_parser_errors)
}

fn reify(expr: Expr) -> Result<Value> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(n)),
        Expr::Symbol(symbol) => Ok(symbol.into()),
        Expr::String(string) => Ok(string.into()),
        Expr::Quote(inner) => {
            let value = reify(*inner)?;
            Value::compound(*symbol::QUOTE, vec![value])
        }
        Expr::Quasiquote(inner) => {
            let value = reify(*inner)?;
            Value::compound(*symbol::QUASIQUOTE, vec![value])
        }
        Expr::Unquote(inner) => {
            let value = reify(*inner)?;
            Value::compound(*symbol::UNQUOTE, vec![value])
        }
        Expr::UnquoteSplicing(inner) => {
            let value = reify(*inner)?;
            Value::compound(*symbol::UNQUOTE_SPLICING, vec![value])
        }
        Expr::List(exprs) => {
            let values = exprs
                .into_iter()
                .map(reify)
                .collect::<Result<Vec<Value>>>()?;
            Value::list(values)
        }
    }
}

pub fn read<S: AsRef<str>>(input: S) -> Result<Value> {
    let expr = parse(input)?;
    reify(expr)
}
