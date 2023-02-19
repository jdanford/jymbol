use im::HashMap;

use crate::{Result, ResultIterator, Symbol};

#[derive(Clone, PartialEq, Debug)]
pub struct Locals {
    vars: Vec<Symbol>,
    indices: HashMap<Symbol, u16>,
}

impl Locals {
    pub fn new() -> Self {
        Locals {
            vars: Vec::new(),
            indices: HashMap::new(),
        }
    }

    pub fn declare(&mut self, var: Symbol) -> Result<u16> {
        if self.indices.contains_key(&var) {
            return Err(format!("`{var}` is already defined"));
        }

        let index = u16::try_from(self.vars.len()).expect("index is out of range");
        self.vars.push(var);
        self.indices.insert(var, index);
        Ok(index)
    }

    pub fn declare_all<'a, I: IntoIterator<Item = &'a Symbol>>(
        &mut self,
        vars: I,
    ) -> Result<Vec<u16>> {
        vars.into_iter().map(|&var| self.declare(var)).try_collect()
    }

    pub fn get_index(&self, var: Symbol) -> Option<u16> {
        self.indices.get(&var).copied()
    }
}
