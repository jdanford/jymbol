use crate::{native, symbol, Env, Function, Result, ResultIterator, Value, VM};

pub fn cons(context: &mut native::Context) -> Result<Value> {
    let [head, tail] = context.as_checked::<2>()?;
    Ok(Value::cons(head, tail))
}

pub fn fn_(context: &mut native::Context) -> Result<Value> {
    let env = context.env.clone().into();
    let [params_list, body] = context.as_checked::<2>()?;
    let params = params_list.into_iter().try_collect()?;
    Ok(Function::new(env, params, body).into())
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
