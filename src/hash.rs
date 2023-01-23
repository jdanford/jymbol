use std::{
    hash::Hasher,
    num::NonZeroU32,
    ops::{Deref, DerefMut},
};

use crate::{symbol, Ref, Result, Symbol, Value, VM};

pub struct ValueHasher<'a, H: Hasher> {
    hasher: H,
    vm: &'a VM,
}

impl<H: Hasher> Deref for ValueHasher<'_, H> {
    type Target = H;

    fn deref(&self) -> &Self::Target {
        &self.hasher
    }
}

impl<H: Hasher> DerefMut for ValueHasher<'_, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hasher
    }
}

fn write_nonzero_u32<H: Hasher>(state: &mut H, n: NonZeroU32) {
    state.write_u32(n.into());
}

impl<'a, H: Hasher> ValueHasher<'a, H> {
    pub fn new(hasher: H, vm: &'a VM) -> Self {
        ValueHasher { hasher, vm }
    }

    pub fn write_value(&mut self, value: &Value) -> Result<()> {
        match *value {
            Value::Number(f) => {
                self.write_number(f);
                Ok(())
            }
            Value::Symbol(symbol) => {
                self.write_symbol(symbol);
                Ok(())
            }
            Value::Ref(ref_) => self.write_ref(ref_),
        }
    }

    fn write_type(&mut self, symbol: Symbol) {
        write_nonzero_u32(&mut **self, symbol.into());
    }

    fn write_number(&mut self, f: f64) {
        self.write_type(*symbol::NUMBER);
        self.write_u64(f.to_bits());
    }

    fn write_symbol(&mut self, symbol: Symbol) {
        self.write_type(*symbol::SYMBOL);
        write_nonzero_u32(&mut **self, symbol.into());
    }

    fn write_ref(&mut self, ref_: Ref) -> Result<()> {
        let (type_, values) = self.vm.heap.load(ref_)?;
        self.write_type(type_);

        for value in values {
            self.write_value(value)?;
        }

        Ok(())
    }
}
