mod code;
mod compile;
pub mod context;
mod locals;

use crate::{
    vm::ClosureType,
    Inst, {Expr, FnId, Result, Symbol, VM},
};

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
                let frame_index = u16::try_from(i + 1).expect("frame index is out of range");
                return Ok((frame_index, index));
            }
        }

        Err(format!("`{sym}` is not defined"))
    }

    fn get_or_create_closure(
        &mut self,
        mut context: Context,
        closure_type: &ClosureType,
        body: &Expr,
    ) -> Result<(Context, FnId)> {
        let fn_id = if let Some(fn_id) = self.vm.id_for_closure_type(closure_type) {
            fn_id
        } else {
            context = self.compile(context, body)?;
            context.code_mut().emit(Inst::Ret);

            let code = context.code_mut().extract();
            self.vm.register_closure(closure_type, code)
        };

        Ok((context, fn_id))
    }
}
