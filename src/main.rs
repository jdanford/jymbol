use jymbol::{Result, VM};

fn main() -> Result<()> {
    let input = "`(list 'a ,b ,@c 1 2 3.0 -4)";
    let mut vm = VM::new();
    let value = vm.read(input)?;
    println!("{:?}", value);
    Ok(())
}
