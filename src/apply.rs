use crate::{Context, Result, Value};

pub trait Apply {
    fn apply(&self, ctx: &mut Context, args: Vec<Value>) -> Result<Value>;
}
