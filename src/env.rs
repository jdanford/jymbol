use std::ops::{Deref, DerefMut};

use anyhow::anyhow;
use gc::{Finalize, Trace};
use im::HashMap;

use crate::{Result, Symbol, Value};

#[derive(Clone, PartialEq, Debug, Finalize)]
pub struct Env {
    pub map: HashMap<Symbol, Value>,
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

    pub fn get(&self, sym: Symbol) -> Result<Value> {
        self.map
            .get(&sym)
            .cloned()
            .ok_or_else(|| anyhow!("`{sym}` is not defined"))
    }

    #[must_use]
    pub fn set<S: Into<Symbol>>(&self, s: S, value: Value) -> Self {
        let map = self.map.update(s.into(), value);
        Env { map }
    }

    #[must_use]
    pub fn set_all(&self, params: &[Symbol], values: &[Value]) -> Self {
        let mut env = self.clone();
        for (&param, value) in params.iter().zip(values) {
            env = env.set(param, value.clone());
        }

        env
    }

    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        let map = self.map.union(other.map);
        Env { map }
    }

    pub fn merge_unique(self, other: Self) -> Result<Self> {
        let intersection = self.map.clone().intersection(other.map.clone());
        if !intersection.is_empty() {
            let existing_vars = intersection.keys().collect::<Vec<_>>();
            let existing_var = existing_vars[0];
            return Err(anyhow!("`{existing_var}` is already defined"));
        }

        Ok(self.merge(other))
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
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
