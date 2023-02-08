use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::{symbol, Compound, Env, Result, Symbol, Value};

pub trait ValueHasher: Hasher {
    fn write_value(&mut self, value: &Value) -> Result<()>;

    fn write_type(&mut self, type_: Symbol) {
        self.write_u32(type_.into());
    }

    fn write_number(&mut self, f: f64) {
        self.write_type(*symbol::NUMBER);
        self.write_u64(f.to_bits());
    }

    fn write_symbol(&mut self, symbol: Symbol) {
        self.write_type(*symbol::SYMBOL);
        self.write_u32(symbol.into());
    }

    fn write_string(&mut self, string: &str) {
        self.write_type(*symbol::STRING);
        self.write(string.as_bytes());
    }

    fn write_env(&mut self, env: &Env) -> Result<()> {
        self.write_type(*symbol::ENV);
        for (symbol, value) in env.iter() {
            self.write_symbol(*symbol);
            self.write_value(value)?;
        }

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

impl<T: Hasher> ValueHasher for T {
    fn write_value(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Number(f) => {
                self.write_number(*f);
                Ok(())
            }
            Value::Symbol(symbol) => {
                self.write_symbol(*symbol);
                Ok(())
            }
            Value::String(string) => {
                self.write_string(string);
                Ok(())
            }
            Value::Env(env) => self.write_env(env),
            Value::Compound(compound) => self.write_compound(compound),
        }
    }
}

pub fn hash(value: &Value) -> Result<u32> {
    let mut hasher = DefaultHasher::new();

    hasher.write_value(value)?;
    let full_hash = hasher.finish();

    #[allow(clippy::cast_possible_truncation)]
    let hash = full_hash as u32;
    Ok(hash)
}
