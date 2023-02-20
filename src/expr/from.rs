use crate::{builtin, try_as_array, Expr, Result, ResultIterator, Symbol, Value};

fn check_var(var: Symbol) -> Result<()> {
    if builtin::VARS.contains(&var) {
        Err(format!("can't bind `{var}`"))
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
                let values = value.iter().map(Expr::value).try_collect()?;
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
            check_var(param)?;
        }

        Ok(Expr::Fn {
            params,
            body: body.into(),
        })
    }

    pub fn let_(var: Symbol, value: Expr, body: Expr) -> Result<Self> {
        check_var(var)?;

        Ok(Expr::Let {
            var,
            value: value.into(),
            body: body.into(),
        })
    }

    pub fn if_(cond: Expr, then: Expr, else_: Expr) -> Self {
        Expr::If {
            cond: cond.into(),
            then: then.into(),
            else_: else_.into(),
        }
    }

    fn try_from_value(value: &Value) -> Result<Expr> {
        match value {
            &Value::Symbol(sym) => Ok(Expr::var(sym)),
            Value::Compound(cons) if cons.is_cons() => {
                let (fn_value, values_list) = cons.as_cons()?;
                let values = values_list.iter().cloned().collect::<Vec<_>>();
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
                if let Some(special_func) = builtin::FUNCTIONS.get(&name) {
                    special_func(values)
                } else {
                    Expr::try_from_call(fn_value, values)
                }
            }
            Value::Closure(_) | Value::NativeFunction(_) => Expr::try_from_call(fn_value, values),
            _ => Err(format!("can't apply {fn_value}")),
        }
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
            .iter()
            .map(|value| value.clone().try_into())
            .try_collect()?;
        let body = body_value.try_into()?;
        Expr::fn_(params, body)
    }

    pub fn try_from_let(values: &[Value]) -> Result<Expr> {
        let [var_value, value, body_value] = try_as_array(values)?;
        let var = var_value.clone().try_into()?;
        let value_expr = value.try_into()?;
        let body = body_value.try_into()?;
        Expr::let_(var, value_expr, body)
    }

    pub fn try_from_if(values: &[Value]) -> Result<Expr> {
        let [cond_value, then_value, else_value] = try_as_array(values)?;
        let cond_expr = cond_value.try_into()?;
        let then_expr = then_value.try_into()?;
        let else_expr = else_value.try_into()?;
        Ok(Expr::if_(cond_expr, then_expr, else_expr))
    }
}

impl TryFrom<&Value> for Expr {
    type Error = String;

    fn try_from(value: &Value) -> Result<Expr> {
        Expr::try_from_value(value)
    }
}
