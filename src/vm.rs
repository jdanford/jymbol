use crate::{symbol, Env, Result, Value};

pub struct VM {}

impl VM {
    #[must_use]
    pub fn new() -> Self {
        VM {}
    }

    pub fn eval(&mut self, env: &Env, value: &Value) -> Result<Value> {
        match value {
            Value::Symbol(symbol) => env
                .get(*symbol)
                .ok_or_else(|| format!("undefined symbol: {symbol:?}")),
            Value::Compound(cons) if cons.type_ == *symbol::CONS => {
                cons.check_len(2)?;
                let func_boxed = &cons.values[0];
                let args = &cons.values[1];
                self.apply(env, func_boxed, args)
            }
            _ => Ok(value.clone()),
        }
    }

    fn apply(&mut self, env: &Env, func_boxed: &Value, args: &Value) -> Result<Value> {
        let func = func_boxed.as_compound()?;
        if func.type_ == *symbol::FN {
            func.check_len(3)?;
            let fn_env = &func.values[0];
            let params = &func.values[1];
            let body = &func.values[2];
            self.apply_fn(env, fn_env, params, args, body)
        } else {
            Err(format!("can't apply {:?}", func.type_))
        }
    }

    fn apply_fn(
        &mut self,
        call_env: &Env,
        fn_env_boxed: &Value,
        params: &Value,
        args: &Value,
        body: &Value,
    ) -> Result<Value> {
        let fn_env = fn_env_boxed.as_env()?;
        let param_results: Vec<Result<Value>> = params.clone().into_iter().collect();
        let arg_results: Vec<Result<Value>> = args.clone().into_iter().collect();

        let expected_arity = param_results.len();
        let actual_arity = arg_results.len();
        if expected_arity != actual_arity {
            return Err(format!(
                "expected {expected_arity} arguments, got {actual_arity}",
            ));
        }

        let mut new_env = (*fn_env).clone();
        for (param_result, arg_result) in param_results.into_iter().zip(arg_results) {
            let param = param_result?;
            let arg = arg_result?;
            if let Value::Symbol(param_symbol) = param {
                let arg = arg.clone();
                let value = self.eval(call_env, &arg)?;
                new_env = new_env.update(param_symbol, value);
            }
        }

        self.eval(&new_env, body)
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
