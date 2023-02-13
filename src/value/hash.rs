use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::{symbol, Symbol, Value};

pub trait ValueHasher: Hasher {
    fn write_value(&mut self, value: &Value) {
        match value {
            Value::Blank => {
                self.write_symbol(*symbol::BLANK);
            }
            Value::Symbol(sym) => {
                self.write_symbol(*symbol::SYMBOL);
                self.write_symbol(*sym);
            }
            Value::RestSymbol(maybe_sym) => {
                self.write_symbol(*symbol::REST_SYMBOL);
                if let Some(sym) = maybe_sym {
                    self.write_symbol(*sym);
                }
            }
            Value::Number(num) => {
                self.write_symbol(*symbol::NUMBER);
                self.write_u64(num.to_bits());
            }
            Value::String(s) => {
                self.write_symbol(*symbol::STRING);
                self.write(s.as_bytes());
            }
            Value::Function(fn_) => {
                self.write_symbol(*symbol::FN);
                self.write_value(&fn_.body);
            }
            Value::NativeFunction(fn_) => {
                self.write_symbol(*symbol::NATIVE_FN);
                self.write_u64(fn_.id);
            }
            Value::Compound(compound) => {
                self.write_symbol(compound.type_);
                for value in &compound.values {
                    self.write_value(value);
                }
            }
        }
    }

    fn write_symbol(&mut self, sym: Symbol) {
        self.write_u32(sym.into());
    }
}

impl<T: Hasher> ValueHasher for T {}

impl Value {
    pub fn hash(&self) -> u32 {
        let mut hasher = DefaultHasher::new();

        hasher.write_value(self);
        let full_hash = hasher.finish();

        #[allow(clippy::cast_possible_truncation)]
        let hash = full_hash as u32;
        hash
    }
}
