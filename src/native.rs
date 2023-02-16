use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    sync::Mutex,
};

use gc::{unsafe_empty_trace, Finalize, Trace};
use once_cell::sync::Lazy;

use crate::{apply::Apply, Arity, Context, Result, ResultIterator, Value};

// pub struct Context<'a> {
//     pub vm: &'a mut VM,
//     pub env: Env,
//     pub args: Vec<Value>,
// }

pub trait Args {
    fn checked<const N: usize>(self) -> Result<[Value; N]>;
}

impl Args for Vec<Value> {
    #[allow(clippy::missing_panics_doc)]
    fn checked<const N: usize>(self) -> Result<[Value; N]> {
        Arity::from(N).check(self.len())?;
        Ok(self.try_into().unwrap())
    }
}

pub type RawFunction = fn(&mut Context<'_>, Vec<Value>) -> Result<Value>;

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
        f.debug_struct("Function")
            .field("id", &self.id)
            .field("arity", &self.arity)
            .field("eval_args", &self.eval_args)
            .finish_non_exhaustive()
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
    fn apply(&self, ctx: &mut Context, args: Vec<Value>) -> Result<Value> {
        self.arity.check(args.len())?;

        if self.eval_args {
            let evaled_args = args.iter().map(|arg| ctx.eval(arg)).try_collect()?;
            (self.f)(ctx, evaled_args)
        } else {
            (self.f)(ctx, args)
        }
    }
}
