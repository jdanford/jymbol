use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::{native, symbol, Compound, Function, Result, Symbol, Value};

pub trait ValueHasher: Hasher {
    fn write_value(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Number(num) => {
                self.write_number(*num);
                Ok(())
            }
            Value::Symbol(sym) => {
                self.write_symbol(*sym);
                Ok(())
            }
            Value::String(s) => {
                self.write_string(s);
                Ok(())
            }
            Value::Function(fn_) => self.write_function(fn_),
            Value::NativeFunction(fn_) => self.write_native_function(fn_),
            Value::Compound(compound) => self.write_compound(compound),
        }
    }

    fn write_type(&mut self, type_: Symbol) {
        self.write_u32(type_.into());
    }

    fn write_number(&mut self, num: f64) {
        self.write_type(*symbol::NUMBER);
        self.write_u64(num.to_bits());
    }

    fn write_symbol(&mut self, sym: Symbol) {
        self.write_type(*symbol::SYMBOL);
        self.write_u32(sym.into());
    }

    fn write_string(&mut self, s: &str) {
        self.write_type(*symbol::STRING);
        self.write(s.as_bytes());
    }

    fn write_function(&mut self, fn_: &Function) -> Result<()> {
        self.write_type(*symbol::FN);
        self.write_value(&fn_.body)?;

        Ok(())
    }

    fn write_native_function(&mut self, fn_: &native::Function) -> Result<()> {
        self.write_type(*symbol::NATIVE_FN);
        self.write_u64(fn_.id);

        Ok(())
    }

    fn write_compound(&mut self, compound: &Compound) -> Result<()> {
        self.write_type(compound.type_);
        for value in &compound.values {
            self.write_value(value)?;
        }

        Ok(())
    }
}

impl<T: Hasher> ValueHasher for T {}

pub fn hash(value: &Value) -> Result<u32> {
    let mut hasher = DefaultHasher::new();

    hasher.write_value(value)?;
    let full_hash = hasher.finish();

    #[allow(clippy::cast_possible_truncation)]
    let hash = full_hash as u32;
    Ok(hash)
}
