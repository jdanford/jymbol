use crate::{apply::Apply, parser, Env, Result, ResultIterator, Value};

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
                .ok_or_else(|| format!("`{sym}` is not defined")),
            Value::Compound(cons) if cons.is_cons() => {
                let (unevaled_fn, args) = cons.as_cons()?;
                let fn_boxed = self.eval(env, &unevaled_fn)?;
                self.apply(env, &fn_boxed, &args)
            }
            _ => Ok(value.clone()),
        }
    }

    pub fn eval_str<S: AsRef<str>>(&mut self, env: &Env, s: S) -> Result<Value> {
        let value = parser::parse(s, parser::value())?;
        self.eval(env, &value)
    }

    fn apply(&mut self, env: &Env, fn_boxed: &Value, args_list: &Value) -> Result<Value> {
        let args = args_list.clone().into_iter().try_collect()?;
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
