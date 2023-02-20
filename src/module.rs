use crate::{builtin, parser, Env, Expr, Result, Symbol, Value, VM};

#[derive(Debug)]
pub struct Module<'a> {
    pub vm: &'a mut VM,
    pub env: Env,
}

impl<'a> Module<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        Module {
            vm,
            env: builtin::env(),
        }
    }

    pub fn set<S: Into<Symbol>>(&mut self, s: S, value: Value) {
        self.env.insert(s.into(), value);
    }

    pub fn eval(&mut self, value: &Value) -> Result<Value> {
        let expr: Expr = value.try_into()?;
        self.vm.eval(&self.env, &expr)
    }

    pub fn eval_str<S: AsRef<str>>(&mut self, s: S) -> Result<Value> {
        let value = parser::parse(s, parser::value())?;
        self.eval(&value)
    }
}
