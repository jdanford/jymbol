use crate::{
    parser::{self, Expr},
    symbol, Result, ResultIterator, Value,
};

pub fn value<S: AsRef<str>>(s: S) -> Result<Value> {
    let expr = parser::parse(s, parser::expr())?;
    reify(expr)
}

pub fn values<S: AsRef<str>>(s: S) -> Result<Vec<Value>> {
    let exprs = parser::parse(s, parser::exprs())?;
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
