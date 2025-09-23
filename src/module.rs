use crate::{Arity, Env, Expr, Result, Symbol, VM, Value, builtin, function::RawFn, parser};

#[derive(Debug)]
pub struct Module<'a> {
    pub vm: &'a mut VM,
    pub env: Env,
}

impl<'a> Module<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        let mut module = Module {
            vm,
            env: Env::new(),
        };

        builtin::define_all(&mut module);
        module
    }

    pub fn set<S: Into<Symbol>>(&mut self, s: S, value: Value) {
        self.env.insert(s.into(), value);
    }

    pub fn set_native<S: Into<Symbol>, A: Into<Arity>>(&mut self, s: S, function: RawFn, arity: A) {
        let fn_id = self.vm.register_native(function, arity);
        self.set(s, Value::NativeFunction(fn_id));
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
