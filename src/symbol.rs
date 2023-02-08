// adapted from `symbol_table/src/global.rs`

use std::{
    fmt::{self, Display, Formatter},
    num::NonZeroU32,
    str::FromStr,
};

use lazy_static::lazy_static;
use symbol_table::SymbolTable;

use crate::Result;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Symbol(symbol_table::Symbol);

impl From<NonZeroU32> for Symbol {
    fn from(n: NonZeroU32) -> Self {
        Self(symbol_table::Symbol::from(n))
    }
}

impl From<Symbol> for NonZeroU32 {
    fn from(symbol: Symbol) -> Self {
        symbol.0.into()
    }
}

impl From<Symbol> for u32 {
    fn from(symbol: Symbol) -> Self {
        NonZeroU32::from(symbol).into()
    }
}

impl TryFrom<u32> for Symbol {
    type Error = String;

    fn try_from(value: u32) -> Result<Self> {
        let i: NonZeroU32 = value
            .try_into()
            .map_err(|_| "expected non-zero value".to_string())?;
        Ok(i.into())
    }
}

impl Symbol {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        s.as_ref().into()
    }

    #[must_use]
    pub fn as_str(&self) -> &'static str {
        (*self).into()
    }
}

lazy_static! {
    static ref GLOBAL_TABLE: SymbolTable = SymbolTable::new();
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Symbol(GLOBAL_TABLE.intern(s))
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Symbol(GLOBAL_TABLE.intern(&s))
    }
}

impl From<&String> for Symbol {
    fn from(s: &String) -> Self {
        Symbol(GLOBAL_TABLE.intern(s))
    }
}

impl FromStr for Symbol {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<Symbol> for &'static str {
    fn from(symbol: Symbol) -> Self {
        GLOBAL_TABLE.resolve(symbol.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

lazy_static! {
    pub static ref NIL: Symbol = Symbol::new("nil");
    pub static ref CONS: Symbol = Symbol::new("cons");
    pub static ref TRUE: Symbol = Symbol::new("true");
    pub static ref FALSE: Symbol = Symbol::new("false");
    pub static ref SYMBOL: Symbol = Symbol::new("symbol");
    pub static ref NUMBER: Symbol = Symbol::new("number");
    pub static ref STRING: Symbol = Symbol::new("string");
    pub static ref REF: Symbol = Symbol::new("ref");
    pub static ref ENV: Symbol = Symbol::new("env");
    pub static ref FN: Symbol = Symbol::new("fn");
    pub static ref NATIVE_FN: Symbol = Symbol::new("native-fn");
    pub static ref QUOTE: Symbol = Symbol::new("quote");
    pub static ref QUASIQUOTE: Symbol = Symbol::new("quasiquote");
    pub static ref UNQUOTE: Symbol = Symbol::new("unquote");
    pub static ref UNQUOTE_SPLICING: Symbol = Symbol::new("unquote-splicing");
}
