use crate::{apply::Apply, parser, Env, Result, ResultIterator, Symbol, Value, VM};

pub struct Context<'a> {
    pub vm: &'a mut VM,
    pub env: Env,
}

impl<'a> Context<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        Context {
            vm,
            env: Env::new(),
        }
    }

    pub fn set<S: Into<Symbol>>(&mut self, s: S, value: Value) {
        self.env.insert(s.into(), value);
    }

    pub fn eval(&mut self, value: &Value) -> Result<Value> {
        match value {
            Value::Blank | Value::RestSymbol(_) => Err(format!("can't evaluate {value}")),
            Value::Symbol(sym) => self
                .env
                .get(*sym)
                .ok_or_else(|| format!("`{sym}` is not defined")),
            Value::Compound(cons) if cons.is_cons() => {
                let (unevaled_fn, args) = cons.as_cons()?;
                let fn_boxed = self.eval(&unevaled_fn)?;
                self.apply(&fn_boxed, &args)
            }
            _ => Ok(value.clone()),
        }
    }

    pub fn eval_str<S: AsRef<str>>(&mut self, s: S) -> Result<Value> {
        let value = parser::parse(s, parser::value())?;
        self.eval(&value)
    }

    fn apply(&mut self, fn_boxed: &Value, args_list: &Value) -> Result<Value> {
        let args = args_list.clone().into_iter().try_collect()?;
        match fn_boxed {
            Value::Function(fn_) => fn_.apply(self, args),
            Value::NativeFunction(fn_) => fn_.apply(self, args),
            _ => Err(format!("can't apply {fn_boxed}")),
        }
    }
}
