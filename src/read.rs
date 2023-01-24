use chumsky::Parser;

use crate::{parser::Expr, symbol, Result, Symbol, Value, VM};

fn collect_parser_errors(errors: Vec<chumsky::error::Simple<char>>) -> String {
    let error_strings: Vec<String> = errors.into_iter().map(|err| err.to_string()).collect();
    error_strings.join("\n")
}

fn parse<S: AsRef<str>>(input: S) -> Result<Expr> {
    let parser = crate::parser::parser();
    parser.parse(input.as_ref()).map_err(collect_parser_errors)
}

impl VM {
    fn alloc_quote(&mut self, type_: Symbol, inner: Value) -> Result<Value> {
        let ref_ = self.heap.alloc(type_, vec![inner])?;
        Ok(ref_.into())
    }

    fn reify(&mut self, expr: Expr) -> Result<Value> {
        match expr {
            Expr::Number(f) => Ok(Value::Number(f)),
            Expr::Symbol(s) => Ok(Value::Symbol(s)),
            Expr::Quote(inner) => {
                let value = self.reify(*inner)?;
                self.alloc_quote(*symbol::QUOTE, value)
            }
            Expr::Quasiquote(inner) => {
                let value = self.reify(*inner)?;
                self.alloc_quote(*symbol::QUASIQUOTE, value)
            }
            Expr::Unquote(inner) => {
                let value = self.reify(*inner)?;
                self.alloc_quote(*symbol::UNQUOTE, value)
            }
            Expr::UnquoteSplicing(inner) => {
                let value = self.reify(*inner)?;
                self.alloc_quote(*symbol::UNQUOTE_SPLICING, value)
            }
            Expr::List(exprs) => {
                let values = exprs
                    .into_iter()
                    .map(|expr| self.reify(expr))
                    .collect::<Result<Vec<Value>>>()?;
                self.heap.alloc_list(values)
            }
        }
    }

    pub fn read<S: AsRef<str>>(&mut self, input: S) -> Result<Value> {
        let expr = parse(input)?;
        self.reify(expr)
    }
}
