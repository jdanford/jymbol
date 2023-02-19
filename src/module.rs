use im::HashMap;

use crate::{parser, Expr, Result, Symbol, Value, VM};

pub struct Module<'a> {
    pub vm: &'a mut VM,
    pub map: HashMap<Symbol, Value>,
}

impl<'a> Module<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        Module {
            vm,
            map: HashMap::new(),
        }
    }

    pub fn set<S: Into<Symbol>>(&mut self, s: S, value: Value) {
        self.map.insert(s.into(), value);
    }

    pub fn eval(&mut self, value: &Value) -> Result<Value> {
        let expr = Expr::try_from(value)?;
        self.vm.eval(&expr)
    }

    pub fn eval_str<S: AsRef<str>>(&mut self, s: S) -> Result<Value> {
        let value = parser::parse(s, parser::value())?;
        self.eval(&value)
    }
}
