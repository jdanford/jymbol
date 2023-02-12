use crate::{Env, Result, Value, VM};

pub trait Apply {
    fn apply(&self, vm: &mut VM, env: &Env, args: &[Value]) -> Result<Value>;
}
