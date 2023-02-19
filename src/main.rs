use jymbol::{parser, symbol, Module, Value, VM};

fn list(values: &[Value]) -> Result<Value, String> {
    Ok(Value::list(values))
}

fn main() -> Result<(), String> {
    let mut vm = VM::new();

    let list_fn_id = vm.register_native(list, ..);
    let list_fn = Value::NativeFunction(list_fn_id);

    let mut module = Module::new(&mut vm);

    module.set(*symbol::NIL, (*symbol::NIL).into());
    module.set(*symbol::TRUE, (*symbol::TRUE).into());
    module.set(*symbol::FALSE, (*symbol::FALSE).into());
    module.set("list", list_fn);

    let input = r#"'(nil false true 'abc 1 -2 3.1416 "hello world")"#;
    let unevaled_value = parser::parse(input, parser::value())?;
    let value = module.eval(&unevaled_value)?;

    println!("{value}");
    Ok(())
}
