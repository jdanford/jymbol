use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
};

use gc::{unsafe_empty_trace, Finalize, Trace};

use crate::{check, Env, Result, ResultIterator, Value, VM};

pub struct Context<'a> {
    pub vm: &'a mut VM,
    pub env: &'a Env,
    pub args: &'a [Value],
}

type NativeFn = fn(&mut Context<'_>) -> Result<Value>;

#[derive(Clone, Finalize)]
pub struct Function {
    pub name: String,
    pub arity: Option<usize>,
    pub eval_args: bool,
    pub inner: Box<NativeFn>,
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

unsafe impl Trace for Function {
    unsafe_empty_trace!();
}

impl Function {
    pub fn new<S: Into<String>>(
        name: S,
        arity: Option<usize>,
        eval_args: bool,
        inner: NativeFn,
    ) -> Function {
        Function {
            name: name.into(),
            arity,
            eval_args,
            inner: inner.into(),
        }
    }

    pub fn apply(&self, vm: &mut VM, env: &Env, args: &[Value]) -> Result<Value> {
        if let Some(expected_arity) = self.arity {
            let actual_arity = args.len();
            check::arity(expected_arity, actual_arity)?;
        }

        if self.eval_args {
            let args = &args.iter().map(|arg| vm.eval(env, arg)).try_collect()?;
            let mut evaled_context = Context { vm, env, args };
            return (self.inner)(&mut evaled_context);
        }

        let mut context = Context { vm, env, args };
        (self.inner)(&mut context)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("native::Function").finish_non_exhaustive()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#native-fn \"{}\" ...)", self.name)
    }
}
