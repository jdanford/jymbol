use crate::{function::Apply, symbol, Env, Result, ResultIterator, Value};

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
            Value::Symbol(sym) => env
                .get(*sym)
                .ok_or_else(|| format!("undefined symbol: {sym}")),
            Value::Compound(cons) if cons.type_ == *symbol::CONS => {
                cons.check_len(2)?;
                let unevaled_fn = &cons.values[0];
                let fn_boxed = self.eval(env, unevaled_fn)?;
                let args = &cons.values[1];
                self.apply(env, &fn_boxed, args)
            }
            _ => Ok(value.clone()),
        }
    }

    fn apply(&mut self, env: &Env, fn_boxed: &Value, args_boxed: &Value) -> Result<Value> {
        let args = args_boxed.clone().into_iter().try_collect()?;
        match fn_boxed {
            Value::Function(fn_) => fn_.apply(self, env, &args),
            Value::NativeFunction(fn_) => fn_.apply(self, env, &args),
            _ => Err(format!("can't apply {fn_boxed}")),
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}
