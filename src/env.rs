use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

use gc::{Finalize, Trace};
use im::HashMap;

use crate::{symbol, Arity, Result, Symbol, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug, Finalize)]
pub struct Env {
    map: HashMap<Symbol, Value>,
}

impl Deref for Env {
    type Target = HashMap<Symbol, Value>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Env {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl Env {
    #[must_use]
    pub fn new() -> Self {
        Env {
            map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn get(&self, sym: Symbol) -> Option<Value> {
        self.map.get(&sym).cloned()
    }

    #[must_use]
    pub fn set<S: Into<Symbol>>(&self, s: S, value: Value) -> Env {
        let map = self.map.update(s.into(), value);
        Env { map }
    }

    #[must_use]
    pub fn set_all(&self, params: &[Symbol], values: &[Value]) -> Env {
        let mut env = self.clone();
        for (param, value) in params.iter().zip(values) {
            env = env.set(*param, value.clone());
        }

        env
    }

    pub fn bind(&self, params: &[Symbol], values: &[Value]) -> Result<Env> {
        let mut env = self.clone();

        match params {
            [named_params @ .., sep, rest_param] if *sep == *symbol::_AMPERSAND => {
                let named_count = named_params.len();
                let arity = Arity::AtLeast(named_count);
                arity.check(values.len())?;

                let (named_values, rest_values) = values.split_at(named_count);
                let rest_list = Value::list(rest_values)?;
                env = env.set_all(named_params, named_values);
                env = env.set(*rest_param, rest_list);
                Ok(env)
            }
            _ => {
                let arity = Arity::Exactly(params.len());
                arity.check(values.len())?;
                Ok(env.set_all(params, values))
            }
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(# env")?;
        for (key, value) in self.iter() {
            write!(f, " ({key} {value})",)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

unsafe impl Trace for Env {
    unsafe fn trace(&self) {
        for value in self.values() {
            value.trace();
        }
    }

    unsafe fn root(&self) {
        for value in self.values() {
            value.root();
        }
    }

    unsafe fn unroot(&self) {
        for value in self.values() {
            value.unroot();
        }
    }

    fn finalize_glue(&self) {
        for value in self.values() {
            value.finalize_glue();
        }
    }
}
