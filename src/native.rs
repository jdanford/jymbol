use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    sync::Mutex,
};

use gc::{unsafe_empty_trace, Finalize, Trace};
use once_cell::sync::Lazy;

use crate::{check, function::Apply, Env, Result, ResultIterator, Value, VM};

pub struct Context<'a> {
    pub vm: &'a mut VM,
    pub env: &'a Env,
    pub args: &'a [Value],
}

pub type RawFunction = fn(&mut Context<'_>) -> Result<Value>;

#[derive(Clone, Finalize)]
pub struct Function {
    pub id: u64,
    pub arity: Option<usize>,
    pub eval_args: bool,
    pub f: Box<RawFunction>,
}

static NEXT_ID: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn next_id() -> u64 {
    let mut next_id = NEXT_ID.lock().unwrap();
    let id = *next_id;
    *next_id += 1;
    id
}

impl Function {
    pub fn new(f: RawFunction, arity: Option<usize>, eval_args: bool) -> Function {
        Function {
            id: next_id(),
            f: f.into(),
            arity,
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
        if let Some(expected_arity) = self.arity {
            let actual_arity = args.len();
            check::arity(expected_arity, actual_arity)?;
        }

        if self.eval_args {
            let args = &args.iter().map(|arg| vm.eval(env, arg)).try_collect()?;
            let mut evaled_context = Context { vm, env, args };
            return (self.f)(&mut evaled_context);
        }

        let mut context = Context { vm, env, args };
        (self.f)(&mut context)
    }
}
