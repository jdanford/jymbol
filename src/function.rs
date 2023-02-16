use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{apply::Apply, unify, Context, Env, Result, ResultIterator, Value};

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
    fn apply(&self, ctx: &mut Context, args: Vec<Value>) -> Result<Value> {
        let evaled_args = args.iter().map(|arg| ctx.eval(arg)).try_collect()?;
        let new_env = unify::slice(&self.params, &evaled_args)?;
        let mut new_ctx = Context {
            vm: ctx.vm,
            env: new_env,
        };
        new_ctx.eval(&self.body)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#fn ...)")
    }
}
