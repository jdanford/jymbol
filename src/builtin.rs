use crate::{native::Args, symbol, Context, Function, Result, ResultIterator, Value, VM};

pub fn cons(_ctx: &mut Context, args: Vec<Value>) -> Result<Value> {
    let [head, tail] = args.checked::<2>()?;
    Ok(Value::cons(head, tail))
}

pub fn fn_(ctx: &mut Context, args: Vec<Value>) -> Result<Value> {
    let env = ctx.env.clone().into();
    let [params_list, body] = args.checked::<2>()?;
    let params = params_list.into_iter().try_collect()?;
    Ok(Function::new(env, params, body).into())
}

pub fn context(vm: &mut VM) -> Result<Context> {
    let mut ctx = Context::new(vm);
    ctx.set(*symbol::NIL, (*symbol::NIL).into());
    ctx.set(*symbol::TRUE, (*symbol::TRUE).into());
    ctx.set(*symbol::FALSE, (*symbol::FALSE).into());

    ctx.set("fn", Value::native_function(fn_, 2, false));
    ctx.set("cons", Value::native_function(cons, 2, false));

    let list_fn = ctx.eval_str("(fn [...values] values)")?;
    ctx.set("list", list_fn);

    Ok(ctx)
}
