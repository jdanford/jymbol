use anyhow::anyhow;

use crate::{Arity, Expr, Inst, Result, Symbol, compiler::context::LoopContext, expr::vars, op};

use super::{Compiler, context::Context};

impl Compiler<'_> {
    pub fn compile(&mut self, mut context: Context, expr: &Expr) -> Result<Context> {
        match expr {
            Expr::Value(value) => {
                context.code.emit(Inst::Value(value.clone()));
                Ok(context)
            }
            &Expr::Var(sym) => {
                let (frame_index, index) = self.lookup(&context, sym)?;
                context.code.emit(Inst::Get(frame_index, index));
                Ok(context)
            }
            Expr::List(exprs) => self.compile_list(context, exprs),
            Expr::Do(exprs) => self.compile_do(context, exprs),
            Expr::UnOp { op, expr } => self.compile_unop(context, *op, expr),
            Expr::BinOp { op, left, right } => self.compile_binop(context, *op, left, right),
            Expr::Call { fn_, args } => self.compile_call(context, fn_, args),
            Expr::Fn { params, body } => self.compile_fn(context, params, body),
            Expr::Let {
                var_expr_pairs,
                body,
            } => self.compile_let(context, var_expr_pairs, body),
            Expr::If {
                cond_expr_pairs,
                else_,
            } => self.compile_if(context, cond_expr_pairs, else_),
            Expr::Loop {
                var_expr_pairs,
                body,
            } => self.compile_loop(context, var_expr_pairs, body),
            Expr::Recur { values } => self.compile_recur(context, values),
        }
    }

    fn compile_list(&mut self, mut context: Context, exprs: &[Expr]) -> Result<Context> {
        for expr in exprs {
            context = self.compile(context, expr)?;
        }

        let value_count = u16::try_from(exprs.len()).unwrap();
        context.code.emit(Inst::List(value_count));

        Ok(context)
    }

    fn compile_do(&mut self, mut context: Context, exprs: &[Expr]) -> Result<Context> {
        for expr in exprs {
            context = self.compile(context, expr)?;
            context.code.emit(Inst::Drop);
        }

        let value_count = u16::try_from(exprs.len()).unwrap();
        context.code.emit(Inst::List(value_count));

        Ok(context)
    }

    fn compile_unop(
        &mut self,
        mut context: Context,
        unop: op::Unary,
        expr: &Expr,
    ) -> Result<Context> {
        context = self.compile(context, expr)?;
        context.code.emit(Inst::UnOp(unop));
        Ok(context)
    }

    fn compile_binop(
        &mut self,
        mut context: Context,
        binop: op::Binary,
        left: &Expr,
        right: &Expr,
    ) -> Result<Context> {
        context = self.compile(context, left)?;
        context = self.compile(context, right)?;
        context.code.emit(Inst::BinOp(binop));
        Ok(context)
    }

    fn compile_call(&mut self, mut context: Context, fn_: &Expr, args: &[Expr]) -> Result<Context> {
        for arg in args {
            context = self.compile(context, arg)?;
        }

        context = self.compile(context, fn_)?;

        let arity = args.len().try_into().unwrap();
        context.code.emit(Inst::Call(arity));
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
        for &var in &closure_vars {
            context = self.compile(context, &Expr::var(var))?;
        }

        let mut new_context = Context::fn_(&context);
        self.contexts.push(context);

        new_context.locals.declare_all(&closure_vars)?;
        new_context.locals.declare_all(params)?;

        let arity = params.len();

        let fn_id;
        (_, fn_id) = self.create_closure(new_context, arity, body)?;

        let closure_value_count = closure_vars.len().try_into().unwrap();
        let mut old_context = self.contexts.pop().unwrap();
        old_context
            .code
            .emit(Inst::Closure(fn_id, closure_value_count));

        Ok(old_context)
    }

    fn compile_let(
        &mut self,
        mut context: Context,
        var_expr_pairs: &[(Symbol, Expr)],
        body: &Expr,
    ) -> Result<Context> {
        for (var, expr) in var_expr_pairs {
            context = self.compile(context, expr)?;
            let index = context.locals.declare(*var)?;
            context.code.emit(Inst::Set(0, index));
        }

        self.compile(context, body)
    }

    fn compile_if(
        &mut self,
        mut context: Context,
        cond_expr_pairs: &[(Expr, Expr)],
        else_: &Expr,
    ) -> Result<Context> {
        let mut entry_points = Vec::new();
        let mut branch_points = Vec::new();
        let mut exit_points = Vec::new();

        for (cond, expr) in cond_expr_pairs {
            let entry_point = context.code.pc();
            entry_points.push(entry_point);

            context = self.compile(context, cond)?;
            let branch_point = context.code.bookmark();
            branch_points.push(branch_point);

            context = self.compile(context, expr)?;
            let exit_point = context.code.bookmark();
            exit_points.push(exit_point);
        }

        let else_entry_point = context.code.pc();
        entry_points.push(else_entry_point);

        context = self.compile(context, else_)?;
        let end = context.code.pc();

        for i in 0..branch_points.len() {
            let next_entry_point = entry_points[i + 1];
            let branch_point = branch_points[i];
            let exit_point = exit_points[i];

            context
                .code
                .patch(branch_point, Inst::JumpIfNot(next_entry_point));
            context.code.patch(exit_point, Inst::Jump(end));
        }

        Ok(context)
    }

    fn compile_loop(
        &mut self,
        mut context: Context,
        var_expr_pairs: &[(Symbol, Expr)],
        body: &Expr,
    ) -> Result<Context> {
        if context.in_loop() {
            return Err(anyhow!("Nested loops aren't supported"));
        }

        let locals_offset = u16::try_from(context.locals.var_count())?;

        for (var, expr) in var_expr_pairs {
            context = self.compile(context, expr)?;
            let index = context.locals.declare(*var)?;
            context.code.emit(Inst::Set(0, index));
        }

        let loop_body_pc = context.code.pc();
        let loop_arity = Arity::Exactly(var_expr_pairs.len());

        context.loop_context = Some(LoopContext {
            frame_offset: 0,
            locals_offset,
            loop_body_pc,
            loop_arity,
        });
        self.compile(context, body)
    }

    fn compile_recur(&mut self, mut context: Context, exprs: &[Expr]) -> Result<Context> {
        let Some(loop_context) = context.loop_context else {
            return Err(anyhow!("Can't use recur outside of loop"));
        };

        let mut set_indices: Vec<u16> = Vec::new();

        let actual_arity = exprs.len();
        let expected_arity = loop_context.loop_arity;
        expected_arity.check(actual_arity)?;

        for (index_usize, expr) in exprs.iter().enumerate() {
            let index = u16::try_from(index_usize)?;
            context = self.compile(context, expr)?;
            set_indices.push(index);
        }

        for &relative_index in set_indices.iter().rev() {
            let index = loop_context.locals_offset + relative_index;
            context
                .code
                .emit(Inst::Set(loop_context.frame_offset, index));
        }

        context.code.emit(Inst::Recur(
            loop_context.frame_offset,
            loop_context.loop_body_pc,
        ));
        Ok(context)
    }
}
