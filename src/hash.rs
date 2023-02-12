use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::{symbol, Result, Symbol, Value};

pub trait ValueHasher: Hasher {
    fn write_value(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Blank => {
                self.write_type(*symbol::BLANK);
                Ok(())
            }
            Value::Symbol(sym) => {
                self.write_type(*symbol::SYMBOL);
                self.write_u32((*sym).into());
                Ok(())
            }
            Value::RestSymbol(maybe_sym) => {
                self.write_type(*symbol::REST_SYMBOL);
                if let Some(sym) = maybe_sym {
                    self.write_u32((*sym).into());
                }
                Ok(())
            }
            Value::Number(num) => {
                self.write_type(*symbol::NUMBER);
                self.write_u64(num.to_bits());
                Ok(())
            }
            Value::String(s) => {
                self.write_type(*symbol::STRING);
                self.write(s.as_bytes());
                Ok(())
            }
            Value::Function(fn_) => {
                self.write_type(*symbol::FN);
                self.write_value(&fn_.body)?;

                Ok(())
            }
            Value::NativeFunction(fn_) => {
                self.write_type(*symbol::NATIVE_FN);
                self.write_u64(fn_.id);

                Ok(())
            }
            Value::Compound(compound) => {
                self.write_type(compound.type_);
                for value in &compound.values {
                    self.write_value(value)?;
                }

                Ok(())
            }
        }
    }

    fn write_type(&mut self, type_: Symbol) {
        self.write_u32(type_.into());
    }
}

impl<T: Hasher> ValueHasher for T {}

impl Value {
    pub fn hash(&self) -> Result<u32> {
        let mut hasher = DefaultHasher::new();

        hasher.write_value(self)?;
        let full_hash = hasher.finish();

        #[allow(clippy::cast_possible_truncation)]
        let hash = full_hash as u32;
        Ok(hash)
    }
}
