use jymbol::{builtin, parser, VM};

fn main() -> Result<(), String> {
    let mut vm = VM::new();
    let mut ctx = builtin::context(&mut vm)?;

    let input = r#"(list nil false true 'abc 1 -2 3.1416 "hello world")"#;
    let unevaled_value = parser::parse(input, parser::value())?;
    let value = ctx.eval(&unevaled_value)?;

    println!("{value}");
    Ok(())
}
