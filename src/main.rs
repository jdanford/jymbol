use jymbol::read;

fn main() -> Result<(), String> {
    let input = "`(list 'a ,b ,@c 1 2 3.0 -4)";
    let value = read(input)?;
    println!("{value}");
    Ok(())
}
