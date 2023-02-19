mod code;
pub mod context;
mod locals;

use crate::{
    expr::vars,
    vm::ClosureType,
    Inst, {Expr, FnId, Result, Symbol, VM},
};

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

    pub fn compile(&mut self, mut context: Context, expr: &Expr) -> Result<Context> {
        match expr {
            Expr::Value(value) => {
                context.code_mut().emit(Inst::Value(value.clone()));
                Ok(context)
            }
            &Expr::Var(sym) => {
                let (frame_index, index) = self.lookup(&context, sym)?;
                context.code_mut().emit(Inst::Get(frame_index, index));
                Ok(context)
            }
            Expr::List(exprs) => self.compile_list(context, exprs),
            Expr::Call { fn_, args } => self.compile_call(context, fn_, args),
            Expr::Fn { params, body } => self.compile_fn(context, params, body),
            Expr::Let { var, value, body } => self.compile_let(context, *var, value, body),
            Expr::If { cond, then, else_ } => self.compile_if(context, cond, then, else_),
        }
    }

    fn compile_list(&mut self, mut context: Context, exprs: &[Expr]) -> Result<Context> {
        for expr in exprs.iter() {
            context = self.compile(context, expr)?;
        }

        let value_count = u16::try_from(exprs.len()).expect("value count is out of range");
        context.code_mut().emit(Inst::List(value_count));

        Ok(context)
    }

    fn compile_call(&mut self, mut context: Context, fn_: &Expr, args: &[Expr]) -> Result<Context> {
        for arg in args.iter() {
            context = self.compile(context, arg)?;
        }

        context = self.compile(context, fn_)?;

        let arity = u16::try_from(args.len()).expect("arity is out of range");
        context.code_mut().emit(Inst::Call(arity));
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

        self.contexts.push(context);

        let mut new_context = Context::new();
        new_context.locals_mut().declare_all(&closure_vars)?;
        new_context.locals_mut().declare_all(params)?;

        let closure_type = ClosureType {
            arity: params.len(),
            local_params: params.to_vec(),
            captured_params: closure_vars.iter().copied().collect(),
            body: body.clone(),
        };

        let (new_new_context, fn_id) =
            self.get_or_create_closure(new_context, &closure_type, body)?;
        new_context = new_new_context; // lol

        let closure_value_count =
            u16::try_from(closure_vars.len()).expect("closure var count is out of range");
        new_context
            .code_mut()
            .emit(Inst::Closure(fn_id, closure_value_count));

        Ok(self.contexts.pop().unwrap())
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
        new_context.code_mut().emit(Inst::Set(0, index));

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
        let branch_pc = context.code_mut().bookmark();

        context = self.compile(context, then)?;
        let then_end_pc = context.code_mut().bookmark();
        let else_start_pc = context.code().pc();

        context = self.compile(context, else_)?;
        let target_pc = context.code().pc();

        context
            .code_mut()
            .patch(branch_pc, Inst::JumpIfNot(else_start_pc));
        context.code_mut().patch(then_end_pc, Inst::Jump(target_pc));

        Ok(context)
    }
}
