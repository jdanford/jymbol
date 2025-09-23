use std::{
    fmt::{self, Debug, Display, Formatter},
    num::NonZeroU32,
    sync::LazyLock,
};

use anyhow::anyhow;
use dumpster::Trace;
use symbol_table::SymbolTable;

use crate::{Error, Result};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Symbol(symbol_table::Symbol);

unsafe impl Trace for Symbol {
    fn accept<V: dumpster::Visitor>(&self, _visitor: &mut V) -> std::result::Result<(), ()> {
        Ok(())
    }
}

impl From<NonZeroU32> for Symbol {
    fn from(n: NonZeroU32) -> Self {
        Self(symbol_table::Symbol::from(n))
    }
}

impl From<Symbol> for NonZeroU32 {
    fn from(sym: Symbol) -> Self {
        sym.0.into()
    }
}

impl From<Symbol> for u32 {
    fn from(sym: Symbol) -> Self {
        NonZeroU32::from(sym).into()
    }
}

impl TryFrom<u32> for Symbol {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        let i: NonZeroU32 = value
            .try_into()
            .map_err(|_| anyhow!("expected non-zero value"))?;
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

static GLOBAL_TABLE: LazyLock<SymbolTable> = LazyLock::new(SymbolTable::new);

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

impl From<Symbol> for &'static str {
    fn from(sym: Symbol) -> Self {
        GLOBAL_TABLE.resolve(sym.0)
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

macro_rules! static_symbol {
    ($name:ident = $value:expr) => {
        pub static $name: ::std::sync::LazyLock<$crate::Symbol> =
            ::std::sync::LazyLock::new(|| $crate::Symbol::new($value));
    };
}

macro_rules! static_symbols {
    ($($name:ident = $value:expr),* $(,)?) => {
        $(static_symbol!($name = $value);)*
    };
}

static_symbols! {
    NIL = "nil",
    CONS = "cons",
    TRUE = "true",
    FALSE = "false",
    BOOLEAN = "boolean",
    SYMBOL = "symbol",
    NUMBER = "number",
    STRING = "string",
    QUOTE = "quote",
    QUASIQUOTE = "quasiquote",
    UNQUOTE = "unquote",
    UNQUOTE_SPLICING = "unquote-splicing",
    FN = "fn",
    NATIVE_FN = "native-fn",
}
