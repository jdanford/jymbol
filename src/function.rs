use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{check, Env, Result, Symbol, Value, VM};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub struct Function {
    pub env: Gc<Env>,
    pub params: Vec<Symbol>,
    pub body: Value,
}

pub trait Apply {
    fn apply(&self, vm: &mut VM, env: &Env, args: &[Value]) -> Result<Value>;
}

impl Function {
    #[must_use]
    pub fn new(env: Gc<Env>, params: Vec<Symbol>, body: Value) -> Self {
        Function { env, params, body }
    }
}

impl Apply for Function {
    fn apply(&self, vm: &mut VM, env: &Env, args: &[Value]) -> Result<Value> {
        let expected_arity = self.params.len();
        let actual_arity = args.len();
        check::arity(expected_arity, actual_arity)?;

        let mut new_env: Env = (*self.env).clone();
        for (param, arg) in self.params.iter().zip(args) {
            let value = vm.eval(env, arg)?;
            new_env = new_env.set(*param, value);
        }

        vm.eval(&new_env, &self.body)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#fn ...)")
    }
}
