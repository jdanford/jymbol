use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{apply::Apply, unify, Env, Result, ResultIterator, Value, VM};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub struct Function {
    pub env: Gc<Env>,
    pub params: Vec<Value>,
    pub body: Value,
}

impl Function {
    pub fn new(env: Gc<Env>, params: Vec<Value>, body: Value) -> Self {
        Function { env, params, body }
    }
}

impl Apply for Function {
    fn apply(&self, vm: &mut VM, env: &Env, args: &[Value]) -> Result<Value> {
        let evaled_args = args.iter().map(|arg| vm.eval(env, arg)).try_collect()?;
        let new_env = unify::slice(&self.params, &evaled_args)?;
        vm.eval(&new_env, &self.body)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#fn ...)")
    }
}
