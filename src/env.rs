use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

use gc::{Finalize, Trace};
use im::HashMap;

use crate::{Symbol, Value};

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
    pub fn set(&self, sym: Symbol, value: Value) -> Env {
        let map = self.map.update(sym, value);
        Env { map }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
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
