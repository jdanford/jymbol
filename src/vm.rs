use crate::{function::Function, symbol, Env, Result, Value};

pub struct VM {
    // ???
}

impl VM {
    #[must_use]
    pub fn new() -> Self {
        VM {
            // ???
        }
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
        if let Value::Function(func) = func_boxed {
            self.apply_function(env, func, args)
        } else {
            Err(format!("can't apply {func_boxed:?}"))
        }
    }

    fn apply_function(&mut self, env: &Env, func: &Function, args: &Value) -> Result<Value> {
        let arg_results: Vec<Result<Value>> = args.clone().into_iter().collect();

        let expected_arity = func.params.len();
        let actual_arity = arg_results.len();
        if expected_arity != actual_arity {
            return Err(format!(
                "expected {expected_arity} arguments, got {actual_arity}",
            ));
        }

        let mut new_env: Env = (*func.env).clone();
        for (param, arg_result) in func.params.iter().zip(arg_results) {
            let arg = arg_result?;
            let value = self.eval(env, &arg)?;
            new_env = new_env.update(*param, value);
        }

        self.eval(&new_env, &func.body)
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
