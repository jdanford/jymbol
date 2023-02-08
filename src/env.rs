use std::fmt::{self, Display, Formatter};

use im::HashMap;

use crate::{Symbol, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Env {
    map: HashMap<Symbol, Value>,
}

impl Env {
    #[must_use]
    pub fn new() -> Self {
        Env {
            map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn get(&self, symbol: Symbol) -> Option<Value> {
        self.map.get(&symbol).copied()
    }

    #[must_use]
    pub fn update(&self, symbol: Symbol, value: Value) -> Env {
        let map = self.map.update(symbol, value);
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
        write!(f, "(env)")
    }
}
