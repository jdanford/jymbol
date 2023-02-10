use crate::{function::Apply, read, symbol, Env, Result, ResultIterator, Value};

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
                let args = &cons.values[1];

                let fn_boxed = self.eval(env, unevaled_fn)?;
                self.apply(env, &fn_boxed, args)
            }
            _ => Ok(value.clone()),
        }
    }

    pub fn eval_str<S: AsRef<str>>(&mut self, env: &Env, s: S) -> Result<Value> {
        let value = read::value(s)?;
        self.eval(env, &value)
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
