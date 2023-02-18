use crate::{Result, ResultIterator, Symbol, Value};

fn try_checked<const N: usize>(values: &[Value]) -> Result<&[Value; N]> {
    let actual_len = values.len();
    values
        .try_into()
        .map_err(|_| format!("expected {N} values, got {actual_len}"))
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Value(Value),
    Symbol(Symbol),
    Fn {
        params: Vec<Symbol>,
        body: Box<Expr>,
    },
    Call {
        fn_: Box<Expr>,
        args: Vec<Expr>,
    },
    Let {
        var: Symbol,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    // Loop {
    //     values: Vec<Expr>,
    //     body: Box<Expr>,
    // },
    // Recur {
    //     values: Vec<Expr>,
    // },
    // Do {
    //     exprs: Vec<Expr>,
    // },
}

impl Expr {
    fn value(value: &Value) -> Self {
        Expr::Value(value.clone())
    }

    fn if_(cond: Expr, then: Expr, else_: Expr) -> Self {
        Expr::If {
            cond: cond.into(),
            then: then.into(),
            else_: else_.into(),
        }
    }

    fn call(fn_: Expr, args: Vec<Expr>) -> Self {
        Expr::Call {
            fn_: fn_.into(),
            args,
        }
    }
}

impl TryFrom<&Value> for Expr {
    type Error = String;

    fn try_from(value: &Value) -> Result<Expr> {
        match value {
            &Value::Symbol(sym) => Ok(Expr::Symbol(sym)),
            Value::Compound(cons) if cons.is_cons() => {
                let (fn_value, values_list) = cons.as_cons()?;
                let values = values_list.into_iter().try_collect()?;
                from_application(&fn_value, &values)
            }
            Value::Compound(quote) if quote.is_quote() => {
                let [value] = try_checked(&quote.values)?;
                Ok(Expr::value(value))
            }
            _ => Ok(Expr::value(value)),
        }
    }
}

fn from_application(fn_value: &Value, values: &[Value]) -> Result<Expr> {
    match fn_value {
        Value::Symbol(sym) => match sym.as_str() {
            "if" => from_if(values),
            "let" => from_let(values),
            "fn" => from_fn(values),
            _ => from_call(fn_value, values),
        },
        Value::Closure(_) | Value::NativeFunction(_) => from_call(fn_value, values),
        _ => Err(format!("can't apply {fn_value}")),
    }
}

fn from_call(fn_value: &Value, args_list: &[Value]) -> Result<Expr> {
    let fn_ = Expr::try_from(fn_value)?;
    let args = args_list.iter().map(Expr::try_from).try_collect()?;
    Ok(Expr::call(fn_, args))
}

fn from_fn(values: &[Value]) -> Result<Expr> {
    let [params_list, body] = try_checked(values)?;
    todo!()
}

fn from_if(values: &[Value]) -> Result<Expr> {
    let [cond_value, then_value, else_value] = try_checked(values)?;
    let cond_expr = Expr::try_from(cond_value)?;
    let then_expr = Expr::try_from(then_value)?;
    let else_expr = Expr::try_from(else_value)?;
    Ok(Expr::if_(cond_expr, then_expr, else_expr))
}

fn from_let(values: &[Value]) -> Result<Expr> {
    todo!()
}
