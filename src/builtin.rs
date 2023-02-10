use crate::{native, Value};

pub fn cons(context: &mut native::Context) -> Result<Value, String> {
    let head = context.args[0].clone();
    let tail = context.args[1].clone();
    Value::cons(head, tail)
}

pub fn list(context: &mut native::Context) -> Result<Value, String> {
    Value::list(context.args)
}
