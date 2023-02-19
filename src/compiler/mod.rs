mod context;
mod locals;

use crate::expr::vars;
use crate::vm::{ClosureType, Inst};
use crate::{symbol, Expr, Result, Symbol, Value, VM};

use self::context::Context;

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

    fn compile(&mut self, mut context: Context, expr: &Expr) -> Result<Context> {
        match expr {
            Expr::Value(value) => {
                context.emit(Inst::Value(value.clone()));
                Ok(context)
            }
            &Expr::Var(sym) => {
                let (frame_index, index) = self.lookup(sym)?;
                context.emit(Inst::Get(frame_index, index));
                Ok(context)
            }
            Expr::List(exprs) => self.compile_list(context, exprs),
            Expr::Fn { params, body } => self.compile_fn(context, params, body),
            Expr::Call { fn_, args } => self.compile_call(context, fn_, args),
            Expr::Let { var, value, body } => self.compile_let(context, *var, value, body),
            Expr::If { cond, then, else_ } => self.compile_if(context, cond, then, else_),
        }
    }

    fn compile_list(&mut self, mut context: Context, exprs: &[Expr]) -> Result<Context> {
        context.emit(Inst::Value(Value::nil()));

        for expr in exprs.iter().rev() {
            context = self.compile(context, expr)?;
            context.emit(Inst::Compound(*symbol::CONS, 2));
        }

        Ok(context)
    }

    fn compile_fn(
        &mut self,
        mut context: Context,
        params: &[Symbol],
        body: &Expr,
    ) -> Result<Context> {
        let fn_vars = vars::list(params);
        let closure_vars = body.free_vars().difference(fn_vars);
        for &var in closure_vars.iter() {
            context = self.compile(context, &Expr::var(var))?;
        }

        let closure_type = ClosureType {
            arity: params.len(),
            local_params: params.to_vec(),
            captured_params: closure_vars.iter().copied().collect(),
            body: body.clone(),
        };
        let fn_id = self.vm.closure_id(&closure_type);
        let closure_value_count = u8::try_from(closure_vars.len())
            .map_err(|_| "closure var count is out of range".to_string())?;
        context.emit(Inst::Closure(fn_id, closure_value_count));

        let mut new_context = context.extend();
        new_context
            .locals_mut()
            .declare_all(params.iter().copied())?;
        new_context.locals_mut().declare_all(closure_vars)?;
        Ok(new_context)
    }

    fn compile_call(&mut self, mut context: Context, fn_: &Expr, args: &[Expr]) -> Result<Context> {
        for arg in args.iter() {
            context = self.compile(context, arg)?;
        }

        context = self.compile(context, fn_)?;

        let arity = u8::try_from(args.len()).map_err(|_| "arity is out of range".to_string())?;
        context.emit(Inst::Call(arity));
        Ok(context)
    }

    fn compile_let(
        &mut self,
        mut context: Context,
        var: Symbol,
        value: &Expr,
        body: &Expr,
    ) -> Result<Context> {
        context = self.compile(context, value)?;

        let mut new_context = context.extend();
        let index = new_context.locals_mut().declare(var)?;
        new_context.emit(Inst::Set(0, index));

        self.compile(new_context, body)
    }

    fn compile_if(
        &mut self,
        mut context: Context,
        cond: &Expr,
        then: &Expr,
        else_: &Expr,
    ) -> Result<Context> {
        context = self.compile(context, cond)?;
        let branch_pc = context.bookmark();

        context = self.compile(context, then)?;
        let then_end_pc = context.bookmark();
        let else_start_pc = context.pc();

        context = self.compile(context, else_)?;
        let target_pc = context.pc();

        context.update(branch_pc, Inst::JumpIfNot(else_start_pc));
        context.update(then_end_pc, Inst::Jump(target_pc));

        Ok(context)
    }
}
