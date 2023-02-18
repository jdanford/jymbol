mod context;
mod expr;
mod locals;

use crate::vm::Instruction;
use crate::{Result, Symbol, VM};

use self::context::Context;
use self::expr::Expr;

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

    fn lookup(&self, sym: Symbol) -> Result<(u16, u16)> {
        for (i, context) in self.contexts.iter().rev().enumerate() {
            if let Some(index) = context.locals().get_index(sym) {
                let frame_index =
                    u16::try_from(i).map_err(|_| "frame index out of range".to_string())?;
                return Ok((frame_index, index));
            }
        }

        Err(format!("`{sym}` is not defined"))
    }

    fn compile(&mut self, mut context: Context, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Value(value) => {
                context.emit(Instruction::Value(value.clone()));
                Ok(())
            }
            &Expr::Symbol(sym) => {
                let (frame_index, index) = self.lookup(sym)?;
                context.emit(Instruction::Get(frame_index, index));
                Ok(())
            }
            Expr::Fn { params, body } => self.compile_fn(context, params, body),
            Expr::Call { fn_, args } => self.compile_call(context, fn_, args),
            Expr::Let { var, value, body } => self.compile_let(context, *var, value, body),
            Expr::If { cond, then, else_ } => self.compile_if(context, cond, then, else_),
        }
    }

    fn compile_fn(&mut self, mut context: Context, params: &[Symbol], body: &Expr) -> Result<()> {
        todo!()
    }

    fn compile_call(&mut self, mut context: Context, fn_: &Expr, args: &[Expr]) -> Result<()> {
        todo!()
    }

    fn compile_let(
        &mut self,
        mut context: Context,
        var: Symbol,
        value: &Expr,
        body: &Expr,
    ) -> Result<()> {
        todo!()
    }

    fn compile_if(
        &mut self,
        mut context: Context,
        cond: &Expr,
        then: &Expr,
        else_: &Expr,
    ) -> Result<()> {
        todo!()
    }
}
