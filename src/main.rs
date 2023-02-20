use jymbol::{Module, Value, VM};

fn list(values: &[Value]) -> Result<Value, String> {
    Ok(Value::list(values))
}

fn main() -> Result<(), String> {
    let mut vm = VM::new();

    let list_fn_id = vm.register_native(list, ..);
    let list_fn = Value::NativeFunction(list_fn_id);

    let mut module = Module::new(&mut vm);
    module.set("list", list_fn);

    let input = r#"(list nil false true 'abc 1 -2 3.1416 "hello world")"#;
    let value = module.eval_str(input)?;

    println!("{value}");
    Ok(())
}
