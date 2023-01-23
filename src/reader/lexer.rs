use std::num::ParseFloatError;

use logos::Logos;

use crate::{Result, Symbol, Value};

pub struct ExtraState {
    pub line: usize,
}

impl Default for ExtraState {
    fn default() -> Self {
        ExtraState { line: 1 }
    }
}

pub type Lexer<'s> = logos::Lexer<'s, Token>;

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = ExtraState)]
pub enum Token {
    #[regex(r"-?\d+(\.\d+)?", number)]
    Number(f64),

    #[regex(r#"[^\-\d\s\(\)"'`,@][^\s\(\)"'`,@]*"#, symbol)]
    Symbol(Symbol),

    #[regex(r#""([^"\n]|\\")""#, string)]
    String(String),

    #[token("'")]
    Quote,

    #[token("`")]
    Quasiquote,

    #[token(",")]
    Unquote,

    #[token(",@")]
    UnquoteSplicing,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    // #[token("[")]
    // LeftSquareBracket,

    // #[token("]")]
    // RightSquareBracket,

    // #[token("{")]
    // LeftCurlyBracket,

    // #[token("}")]
    // RightCurlyBracket,
    //
    #[token("\n", newline)]
    Newline,

    #[error]
    #[regex(r"[\s]+", logos::skip)]
    Error,
}

impl TryFrom<Token> for Value {
    type Error = String;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Number(f) => Ok(Value::Number(f)),
            Token::Symbol(symbol) => Ok(Value::Symbol(symbol)),
            Token::String(_string) => unimplemented!(),
            _ => Err("???".to_string()),
        }
    }
}

// impl Token {
//     pub fn is_atom(&self) -> bool {
//         matches!(
//             *self,
//             Token::Number(_) | Token::Symbol(_) | Token::String(_)
//         )
//     }
// }

fn number(lex: &mut Lexer) -> Result<f64> {
    let slice = lex.slice();
    slice
        .parse()
        .map_err(|err: ParseFloatError| err.to_string())
}

fn symbol(lex: &mut Lexer) -> Symbol {
    let slice = lex.slice();
    Symbol::new(slice)
}

fn string(lex: &mut Lexer) -> Result<String> {
    let slice = lex.slice();
    let start = 1;
    let end = slice.len() - 1;
    let contents = &slice[start..end];
    unescaper::unescape(contents).map_err(|err| err.to_string())
}

fn newline(lexer: &mut Lexer<'_>) {
    lexer.extras.line += 1;
}
