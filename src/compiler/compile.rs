use crate::{expr::vars, op, vm::ClosureType, Expr, Inst, Result, Symbol};

use super::{context::Context, Compiler};

impl Compiler<'_> {
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
        }
    }

    fn compile_list(&mut self, mut context: Context, exprs: &[Expr]) -> Result<Context> {
        for expr in exprs {
            context = self.compile(context, expr)?;
        }

        let value_count = u16::try_from(exprs.len()).unwrap();
        context.code_mut().emit(Inst::List(value_count));

        Ok(context)
    }

    fn compile_do(&mut self, mut context: Context, exprs: &[Expr]) -> Result<Context> {
        for expr in exprs {
            context = self.compile(context, expr)?;
            context.code_mut().emit(Inst::Drop);
        }

        let value_count = u16::try_from(exprs.len()).unwrap();
        context.code_mut().emit(Inst::List(value_count));

        Ok(context)
    }

    fn compile_unop(
        &mut self,
        mut context: Context,
        unop: op::Unary,
        expr: &Expr,
    ) -> Result<Context> {
        context = self.compile(context, expr)?;
        context.code_mut().emit(Inst::UnOp(unop));
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
        context.code_mut().emit(Inst::BinOp(binop));
        Ok(context)
    }

    fn compile_call(&mut self, mut context: Context, fn_: &Expr, args: &[Expr]) -> Result<Context> {
        for arg in args {
            context = self.compile(context, arg)?;
        }

        context = self.compile(context, fn_)?;

        let arity = args.len().try_into().unwrap();
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
        for &var in &closure_vars {
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

        let fn_id;
        (new_context, fn_id) = self.get_or_create_closure(new_context, &closure_type, body)?;

        let closure_value_count = closure_vars.len().try_into().unwrap();
        new_context
            .code_mut()
            .emit(Inst::Closure(fn_id, closure_value_count));

        Ok(self.contexts.pop().unwrap())
    }

    fn compile_let(
        &mut self,
        context: Context,
        var_expr_pairs: &[(Symbol, Expr)],
        body: &Expr,
    ) -> Result<Context> {
        let mut extended_context = context.extend();
        for (var, expr) in var_expr_pairs {
            extended_context = self.compile(extended_context, expr)?;
            let index = extended_context.locals_mut().declare(*var)?;
            extended_context.code_mut().emit(Inst::Set(0, index));
        }

        self.compile(extended_context, body)
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
            let entry_point = context.code().pc();
            entry_points.push(entry_point);

            context = self.compile(context, cond)?;
            let branch_point = context.code_mut().bookmark();
            branch_points.push(branch_point);

            context = self.compile(context, expr)?;
            let exit_point = context.code_mut().bookmark();
            exit_points.push(exit_point);
        }

        let else_entry_point = context.code().pc();
        entry_points.push(else_entry_point);

        context = self.compile(context, else_)?;
        let end = context.code().pc();

        for i in 0..branch_points.len() {
            let next_entry_point = entry_points[i + 1];
            let branch_point = branch_points[i];
            let exit_point = exit_points[i];

            context
                .code_mut()
                .patch(branch_point, Inst::JumpIfNot(next_entry_point));
            context.code_mut().patch(exit_point, Inst::Jump(end));
        }

        Ok(context)
    }
}
