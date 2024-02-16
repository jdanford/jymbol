use crate::{symbol, try_as_array, Module, Result, Value};

fn type_(values: &[Value]) -> Result<Value> {
    let [value] = try_as_array(values)?;
    Ok(value.type_().into())
}

#[allow(clippy::unnecessary_wraps)]
fn list(values: &[Value]) -> Result<Value> {
    Ok(Value::list(values))
}

pub fn define_all(module: &mut Module) {
    module.set(*symbol::NIL, Value::nil());
    module.set(*symbol::TRUE, Value::true_());
    module.set(*symbol::FALSE, Value::false_());

    module.set_native("type", type_, 1);
    module.set_native("list", list, ..);
}
