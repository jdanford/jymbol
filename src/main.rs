use jymbol::{builtin, parser, VM};

fn main() -> Result<(), String> {
    let mut vm = VM::new();
    let env = builtin::env(&mut vm)?;

    let input = "(list nil false true 'abc 1 -2 3.0)";
    let unevaled_value = parser::parse(input, parser::value())?;
    let value = vm.eval(&env, &unevaled_value)?;

    println!("{value}");
    Ok(())
}
