use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    sync::Mutex,
};

use gc::{unsafe_empty_trace, Finalize, Trace};
use once_cell::sync::Lazy;

use crate::{function::Apply, Arity, Env, Result, ResultIterator, Value, VM};

pub struct Context<'a> {
    pub vm: &'a mut VM,
    pub env: &'a Env,
    pub args: &'a [Value],
}

pub type RawFunction = fn(&mut Context<'_>) -> Result<Value>;

#[derive(Clone, Finalize)]
pub struct Function {
    pub id: u64,
    pub f: Box<RawFunction>,
    pub arity: Arity,
    pub eval_args: bool,
}

static NEXT_ID: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn next_id() -> u64 {
    let mut next_id = NEXT_ID.lock().unwrap();
    let id = *next_id;
    *next_id += 1;
    id
}

impl Function {
    pub fn new<A: Into<Arity>>(f: RawFunction, arity: A, eval_args: bool) -> Function {
        Function {
            id: next_id(),
            f: f.into(),
            arity: arity.into(),
            eval_args,
        }
    }
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

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("native::Function").finish_non_exhaustive()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#native-fn ...)")
    }
}

unsafe impl Trace for Function {
    unsafe_empty_trace!();
}

impl Apply for Function {
    fn apply(&self, vm: &mut VM, env: &Env, args: &[Value]) -> Result<Value> {
        self.arity.check(args.len())?;

        if self.eval_args {
            let args = &args.iter().map(|arg| vm.eval(env, arg)).try_collect()?;
            let mut evaled_context = Context { vm, env, args };
            return (self.f)(&mut evaled_context);
        }

        let mut context = Context { vm, env, args };
        (self.f)(&mut context)
    }
}
