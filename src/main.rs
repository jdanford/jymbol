use jymbol::{Error, Module, VM};

fn main() -> Result<(), Error> {
    let mut vm = VM::new();
    let mut module = Module::new(&mut vm);

    let input = r#"(list nil false true 'abc 1 -2 3.1416 "hello world")"#;
    let value = module.eval_str(input)?;

    println!("{value}");
    Ok(())
}
