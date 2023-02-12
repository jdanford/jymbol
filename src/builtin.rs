use crate::{native, symbol, Env, Function, Result, ResultIterator, Value, VM};

pub fn cons(context: &mut native::Context) -> Result<Value> {
    let head = context.args[0].clone();
    let tail = context.args[1].clone();
    Ok(Value::cons(head, tail))
}

pub fn fn_(context: &mut native::Context) -> Result<Value> {
    let params_list = context.args[0].clone();
    let body = context.args[1].clone();
    let params = params_list.into_iter().try_collect()?;
    Ok(Function::new(context.env.clone().into(), params, body).into())
}

pub fn env(vm: &mut VM) -> Result<Env> {
    let mut env = Env::new();
    env = env.set(*symbol::NIL, (*symbol::NIL).into());
    env = env.set(*symbol::TRUE, (*symbol::TRUE).into());
    env = env.set(*symbol::FALSE, (*symbol::FALSE).into());

    env = env.set("fn", Value::native_function(fn_, 2, false));
    env = env.set("cons", Value::native_function(cons, 2, false));
    env = env.set("list", vm.eval_str(&env, "(fn [...values] values)")?);

    Ok(env)
}
