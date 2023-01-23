use std::ops::{Deref, DerefMut};

use logos::{Lexer, Logos};

use crate::{Value, VM};

use super::{lexer::Token, Error, Result};

pub struct Reader<'s, 'vm> {
    lexer: Lexer<'s, Token>,
    vm: &'vm VM,
}

impl<'s> Deref for Reader<'s, '_> {
    type Target = Lexer<'s, Token>;

    fn deref(&self) -> &Self::Target {
        &self.lexer
    }
}

impl<'s> DerefMut for Reader<'s, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lexer
    }
}

impl<'s, 'vm> Reader<'s, 'vm> {
    #[must_use]
    pub fn new(input: &'s str, vm: &'vm VM) -> Reader<'s, 'vm> {
        let lexer = Token::lexer(input);
        Reader { lexer, vm }
    }

    fn error(&self, message: String) -> Error {
        Error {
            message,
            line: self.extras.line,
            span: self.span(),
        }
    }

    fn fail<T>(&self, message: String) -> Result<T> {
        Err(self.error(message))
    }

    fn unexpected<T>(&self) -> Result<T> {
        self.fail(format!("unexpected: {}", self.slice()))
    }

    fn unrecognized<T>(&self) -> Result<T> {
        self.fail(format!("unrecognized characters: {}", self.slice()))
    }

    fn parse_atom(&self, token: Token) -> Result<Value> {
        token.try_into().map_err(|msg| self.error(msg))
    }
}
