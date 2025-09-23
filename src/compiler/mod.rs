mod code;
mod compile;
pub mod context;
mod locals;

use anyhow::anyhow;

use crate::{Arity, Expr, FnId, Inst, Result, Symbol, VM};

use self::context::Context;

#[derive(Debug)]
pub struct Compiler<'a> {
    pub vm: &'a mut VM,
    pub contexts: Vec<Context>,
}

impl<'a> Compiler<'a> {
    pub fn new(vm: &'a mut VM) -> Self {
        Compiler {
            vm,
            contexts: Vec::new(),
        }
    }

    fn lookup(&self, context: &Context, sym: Symbol) -> Result<(u16, u16)> {
        if let Some(index) = context.locals().get_index(sym) {
            return Ok((0, index));
        }

        for (i, context) in self.contexts.iter().rev().enumerate() {
            if let Some(index) = context.locals().get_index(sym) {
                let frame_index = u16::try_from(i + 1).unwrap();
                return Ok((frame_index, index));
            }
        }

        Err(anyhow!("`{sym}` is not defined"))
    }

    fn create_closure<A: Into<Arity>>(
        &mut self,
        mut context: Context,
        arity: A,
        body: &Expr,
    ) -> Result<(Context, FnId)> {
        context = self.compile(context, body)?;
        context.code_mut().emit(Inst::Return);

        let code = context.code_mut().extract();
        let fn_id = self.vm.register_closure(arity, code);

        Ok((context, fn_id))
    }
}
