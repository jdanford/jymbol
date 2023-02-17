use crate::{parser, Env, Result, Symbol, Value, VM};

pub struct Module<'a> {
    pub vm: &'a mut VM,
    pub env: Env,
}

impl<'a> Module<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        Module {
            vm,
            env: Env::new(),
        }
    }

    pub fn set<S: Into<Symbol>>(&mut self, s: S, value: Value) {
        self.env.insert(s.into(), value);
    }

    pub fn eval(&mut self, _value: &Value) -> Result<Value> {
        Err("not implemented".to_string())
    }

    pub fn eval_str<S: AsRef<str>>(&mut self, s: S) -> Result<Value> {
        let value = parser::parse(s, parser::value())?;
        self.eval(&value)
    }
}
