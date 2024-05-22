use anyhow::anyhow;

use crate::{op, special, try_as_array, Error, Expr, Result, ResultIterator, Symbol, Value};

fn check_var_is_valid(var: Symbol) -> Result<()> {
    if special::VARS.contains(&var) {
        Err(anyhow!("can't bind reserved symbol `{var}`"))
    } else {
        Ok(())
    }
}

impl Expr {
    #[must_use]
    pub fn var<S: Into<Symbol>>(s: S) -> Self {
        Expr::Var(s.into())
    }

    pub fn value(value: &Value) -> Result<Self> {
        match value {
            Value::Compound(cons) if cons.is_cons() => {
                let values = value.into_iter().map(Expr::value).try_collect()?;
                Ok(Expr::List(values))
            }
            _ => Ok(Expr::Value(value.clone())),
        }
    }

    pub fn call(fn_: Expr, args: Vec<Expr>) -> Self {
        Expr::Call {
            fn_: fn_.into(),
            args,
        }
    }

    pub fn fn_(params: Vec<Symbol>, body: Expr) -> Result<Self> {
        for &param in &params {
            check_var_is_valid(param)?;
        }

        Ok(Expr::Fn {
            params,
            body: body.into(),
        })
    }

    pub fn let_(var_expr_pairs: Vec<(Symbol, Expr)>, body: Expr) -> Result<Self> {
        for &(var, _) in &var_expr_pairs {
            check_var_is_valid(var)?;
        }

        Ok(Expr::Let {
            var_expr_pairs,
            body: Box::new(body),
        })
    }

    pub fn if_(cond_expr_pairs: Vec<(Expr, Expr)>, else_: Expr) -> Self {
        Expr::If {
            cond_expr_pairs,
            else_: Box::new(else_),
        }
    }

    fn try_from_value(value: &Value) -> Result<Expr> {
        match value {
            &Value::Symbol(sym) => Ok(Expr::var(sym)),
            Value::Compound(cons) if cons.is_cons() => {
                let (fn_value, values_list) = cons.as_cons()?;
                let values = values_list.into_iter().cloned().collect::<Vec<_>>();
                Expr::try_from_application(&fn_value, &values)
            }
            Value::Compound(quote) if quote.is_quote() => {
                let [value] = try_as_array(&quote.values)?;
                Expr::value(value)
            }
            _ => Expr::value(value),
        }
    }

    pub fn try_from_application(fn_value: &Value, values: &[Value]) -> Result<Expr> {
        match fn_value {
            Value::Symbol(sym) => {
                let name = sym.as_str();
                if let Some(special_func) = special::FUNCTIONS.get(&name) {
                    special_func(values)
                } else {
                    Expr::try_from_call(fn_value, values)
                }
            }
            Value::Closure(_) | Value::NativeFunction(_) => Expr::try_from_call(fn_value, values),
            _ => Err(anyhow!("can't apply {fn_value}")),
        }
    }

    pub fn try_from_unop(op: op::Unary, values: &[Value]) -> Result<Expr> {
        let [value] = try_as_array(values)?;
        let expr = value.try_into()?;
        Ok(Expr::UnOp {
            op,
            expr: Box::new(expr),
        })
    }

    pub fn try_from_binop(op: op::Binary, values: &[Value]) -> Result<Expr> {
        let [left_value, right_value] = try_as_array(values)?;
        let left = left_value.try_into()?;
        let right = right_value.try_into()?;
        Ok(Expr::BinOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn try_from_do(values: &[Value]) -> Result<Expr> {
        let exprs = values.iter().map(Expr::try_from_value).try_collect()?;
        Ok(Expr::Do(exprs))
    }

    pub fn try_from_call(fn_value: &Value, args_list: &[Value]) -> Result<Expr> {
        let fn_ = Expr::try_from(fn_value)?;
        let args = args_list.iter().map(Expr::try_from).try_collect()?;
        Ok(Expr::call(fn_, args))
    }

    pub fn try_from_fn(values: &[Value]) -> Result<Expr> {
        let [params_list, body_value] = try_as_array(values)?;
        let params = params_list
            .into_iter()
            .map(|value| value.clone().try_into())
            .try_collect()?;
        let body = body_value.try_into()?;
        Expr::fn_(params, body)
    }

    pub fn try_from_let(values: &[Value]) -> Result<Expr> {
        match values {
            [var_value_pairs @ .., body_value] if var_value_pairs.len() >= 2 => {
                let var_expr_pairs = var_value_pairs
                    .chunks_exact(2)
                    .map(Expr::try_from_var_expr_pair)
                    .try_collect()?;
                let body = body_value.try_into()?;
                Expr::let_(var_expr_pairs, body)
            }
            _ => Err(anyhow!("malformed `let` expression")),
        }
    }

    fn try_from_var_expr_pair(values: &[Value]) -> Result<(Symbol, Expr)> {
        if let [var_value, value] = values {
            Result::Ok((var_value.clone().try_into()?, value.try_into()?))
        } else {
            Err(anyhow!("malformed `let` expression"))
        }
    }

    pub fn try_from_if(values: &[Value]) -> Result<Expr> {
        match values {
            [cond_value_pairs @ .., else_value] if cond_value_pairs.len() >= 2 => {
                let cond_expr_pairs = cond_value_pairs
                    .chunks_exact(2)
                    .map(Expr::try_from_cond_expr_pair)
                    .try_collect()?;
                let else_expr = else_value.try_into()?;
                Ok(Expr::if_(cond_expr_pairs, else_expr))
            }
            _ => Err(anyhow!("malformed `if` expression")),
        }
    }

    fn try_from_cond_expr_pair(values: &[Value]) -> Result<(Expr, Expr)> {
        if let [cond_value, expr_value] = values {
            Result::Ok((cond_value.try_into()?, expr_value.try_into()?))
        } else {
            Err(anyhow!("malformed `if` expression"))
        }
    }
}

impl TryFrom<&Value> for Expr {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Expr> {
        Expr::try_from_value(value)
    }
}
