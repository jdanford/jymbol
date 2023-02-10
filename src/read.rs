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
            let values = exprs
                .into_iter()
                .map(reify)
                .collect::<Result<Vec<Value>>>()?;
            Value::list(&values)
        }
    }
}

pub fn read<S: AsRef<str>>(input: S) -> Result<Value> {
    let expr = parse(input)?;
    reify(expr)
}
