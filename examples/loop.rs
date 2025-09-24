use jymbol::{Error, Module, VM};

fn main() -> Result<(), Error> {
    let mut vm = VM::new();
    let mut module = Module::new(&mut vm);

    let input = r#"(loop [n 1e6] (if ($eq n 0) 'done (recur ($sub n 1))))"#;
    let value = module.eval_str(input)?;
    println!("{value}");
    Ok(())
}
