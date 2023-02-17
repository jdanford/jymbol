use std::hash::{Hash, Hasher};

use crate::{symbol, Symbol, Value};

pub trait ValueHasher: Hasher {
    fn write_symbol(&mut self, sym: Symbol) {
        self.write_u32(sym.into());
    }
}

impl<T: Hasher> ValueHasher for T {}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Blank => {
                state.write_symbol(*symbol::BLANK);
            }
            Value::Symbol(sym) => {
                state.write_symbol(*symbol::SYMBOL);
                state.write_symbol(*sym);
            }
            Value::RestSymbol(maybe_sym) => {
                state.write_symbol(*symbol::REST_SYMBOL);
                if let Some(sym) = maybe_sym {
                    state.write_symbol(*sym);
                }
            }
            Value::Number(num) => {
                state.write_symbol(*symbol::NUMBER);
                state.write_u64(num.to_bits());
            }
            Value::String(s) => {
                state.write_symbol(*symbol::STRING);
                state.write(s.as_bytes());
            }
            Value::Closure(closure) => {
                state.write_symbol(*symbol::FN);
                state.write_u32(closure.fn_id.into());
            }
            Value::NativeFunction(fn_id) => {
                state.write_symbol(*symbol::NATIVE_FN);
                state.write_u32((*fn_id).into());
            }
            Value::Compound(compound) => {
                state.write_symbol(compound.type_);
                for value in &compound.values {
                    value.hash(state);
                }
            }
        }
    }
}
