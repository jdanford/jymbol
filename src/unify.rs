use crate::{Env, Result, ResultIterator, Value};

pub fn single(pattern: &Value, value: &Value) -> Result<Env> {
    match (pattern, value) {
        (Value::Blank, _) => Ok(Env::new()),
        (Value::Symbol(sym), _) => Ok(Env::new().set(*sym, value.clone())),
        #[allow(clippy::float_cmp)]
        (Value::Number(a), Value::Number(b)) if a == b => Ok(Env::new()),
        (Value::String(a), Value::String(b)) if a == b => Ok(Env::new()),
        (Value::Compound(pattern_compound), Value::Compound(value_compound))
            if pattern_compound.is_cons() && value_compound.is_cons() =>
        {
            let patterns = pattern.clone().into_iter().try_collect()?;
            list(&patterns, value)
        }
        _ => Err(format!("can't match {pattern} with {value}")),
    }
}

pub fn list(patterns: &[Value], values_list: &Value) -> Result<Env> {
    match (patterns, values_list) {
        ([], sym) if *sym == Value::nil() => Ok(Env::new()),
        ([Value::RestSymbol(None)], _) => Ok(Env::new()),
        ([Value::RestSymbol(Some(sym))], _) => Ok(Env::new().set(*sym, values_list.clone())),
        ([pattern, patterns @ ..], Value::Compound(cons)) => {
            let (value, values) = cons.as_cons()?;
            let head_env = single(pattern, &value)?;
            let tail_env = list(patterns, &values)?;
            head_env.merge_unique(tail_env)
        }
        _ => Err(format!("can't match {patterns:?} with {values_list}")),
    }
}

pub fn slice(patterns: &[Value], values: &[Value]) -> Result<Env> {
    match (patterns, values) {
        ([], []) | ([Value::RestSymbol(None)], _) => Ok(Env::new()),
        ([Value::RestSymbol(Some(sym))], _) => {
            let list = Value::list(values);
            Ok(Env::new().set(*sym, list))
        }
        ([pattern, patterns @ ..], [value, values @ ..]) => {
            let head_env = single(pattern, value)?;
            let tail_env = slice(patterns, values)?;
            head_env.merge_unique(tail_env)
        }
        _ => {
            let values_list = Value::list(values);
            Err(format!("can't match {patterns:?} with {values_list}"))
        }
    }
}
