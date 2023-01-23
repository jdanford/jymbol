use lazy_static::lazy_static;

pub type Symbol = symbol_table::GlobalSymbol;

lazy_static! {
    pub static ref NIL: Symbol = Symbol::new("nil");
    pub static ref CONS: Symbol = Symbol::new("cons");
    pub static ref TRUE: Symbol = Symbol::new("true");
    pub static ref FALSE: Symbol = Symbol::new("false");
    pub static ref SYMBOL: Symbol = Symbol::new("symbol");
    pub static ref NUMBER: Symbol = Symbol::new("number");
    pub static ref STRING: Symbol = Symbol::new("string");
    pub static ref REF: Symbol = Symbol::new("ref");
    pub static ref FN: Symbol = Symbol::new("fn");
    pub static ref NATIVE_FN: Symbol = Symbol::new("native-fn");
    pub static ref QUOTE: Symbol = Symbol::new("quote");
    pub static ref QUASIQUOTE: Symbol = Symbol::new("quasiquote");
    pub static ref UNQUOTE: Symbol = Symbol::new("unquote");
    pub static ref UNQUOTE_SPLICING: Symbol = Symbol::new("unquote-splicing");
}
