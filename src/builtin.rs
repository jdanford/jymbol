use crate::{native, read, symbol, Env, Result, ResultIterator, Value, VM};

pub fn cons(context: &mut native::Context) -> Result<Value> {
    let head = context.args[0].clone();
    let tail = context.args[1].clone();
    Value::cons(head, tail)
}

pub fn fn_(context: &mut native::Context) -> Result<Value> {
    let params_boxed = context.args[0].clone();
    let body = context.args[1].clone();
    let params = params_boxed
        .into_iter()
        .map(|value| value?.try_into())
        .try_collect()?;
    Value::function(context.env.clone().into(), params, body)
}

pub fn env(vm: &mut VM) -> Result<Env> {
    let mut env = Env::new();
    env = env.set(*symbol::NIL, (*symbol::NIL).into());
    env = env.set(*symbol::FALSE, (*symbol::FALSE).into());
    env = env.set(*symbol::TRUE, (*symbol::TRUE).into());

    env = env.set("fn", Value::native_function(fn_, 2, false)?);
    env = env.set("cons", Value::native_function(cons, 2, false)?);

    let list = vm.eval(&env, &read("(fn [& values] values)")?)?;
    env = env.set("list", list);

    Ok(env)
}
