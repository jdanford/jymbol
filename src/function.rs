use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{Env, Result, ResultIterator, Symbol, Value, VM};

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
        let evaled_args = args.iter().map(|arg| vm.eval(env, arg)).try_collect()?;
        let new_env = env.bind(&self.params, &evaled_args)?;
        vm.eval(&new_env, &self.body)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#fn ...)")
    }
}
