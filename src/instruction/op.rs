use std::{cmp, ops};

use crate::{Result, Value};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Unary {
    Abs,
    Neg,
    Sqrt,
    Trunc,
    Fract,
    Round,
    Floor,
    Ceil,
    Not,
}

impl Unary {
    pub fn apply(self, value: &Value) -> Result<Value> {
        match self {
            Unary::Abs => unary_float_op(value, f64::abs),
            Unary::Neg => unary_float_op(value, ops::Neg::neg),
            Unary::Sqrt => unary_float_op(value, f64::sqrt),
            Unary::Trunc => unary_float_op(value, f64::trunc),
            Unary::Fract => unary_float_op(value, f64::fract),
            Unary::Round => unary_float_op(value, f64::round),
            Unary::Floor => unary_float_op(value, f64::floor),
            Unary::Ceil => unary_float_op(value, f64::ceil),
            Unary::Not => unary_int_op(value, ops::Not::not),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Binary {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Binary {
    #[allow(clippy::cast_possible_truncation, clippy::float_cmp)]
    pub fn apply(self, a: &Value, b: &Value) -> Result<Value> {
        match self {
            Binary::Add => binary_float_op(a, b, ops::Add::add),
            Binary::Sub => binary_float_op(a, b, ops::Sub::sub),
            Binary::Mul => binary_float_op(a, b, ops::Mul::mul),
            Binary::Div => binary_float_op(a, b, ops::Div::div),
            Binary::Mod => binary_float_op(a, b, ops::Rem::rem),
            Binary::Pow => binary_float_op(a, b, f64::powf),
            Binary::Shl => binary_int_op(a, b, ops::Shl::shl),
            Binary::Shr => binary_int_op(a, b, ops::Shr::shr),
            Binary::And => binary_int_op(a, b, ops::BitAnd::bitand),
            Binary::Or => binary_int_op(a, b, ops::BitOr::bitor),
            Binary::Xor => binary_int_op(a, b, ops::BitXor::bitxor),
            Binary::Eq => bool_op(a, b, cmp::PartialEq::eq),
            Binary::Ne => bool_op(a, b, cmp::PartialEq::ne),
            Binary::Lt => bool_op(a, b, cmp::PartialOrd::lt),
            Binary::Gt => bool_op(a, b, cmp::PartialOrd::gt),
            Binary::Le => bool_op(a, b, cmp::PartialOrd::le),
            Binary::Ge => bool_op(a, b, cmp::PartialOrd::ge),
        }
    }
}

fn unary_float_op<F: Fn(f64) -> f64>(value: &Value, f: F) -> Result<Value> {
    let num = value.as_number()?;
    let num_result = f(num);
    Ok(num_result.into())
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn unary_int_op<F: Fn(i64) -> i64>(value: &Value, f: F) -> Result<Value> {
    let num = value.as_number()?;
    let num_result = f(num as i64) as f64;
    Ok(num_result.into())
}

fn binary_float_op<F: Fn(f64, f64) -> f64>(a: &Value, b: &Value, f: F) -> Result<Value> {
    let num_a = a.as_number()?;
    let num_b = b.as_number()?;
    let num_result = f(num_a, num_b);
    Ok(num_result.into())
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn binary_int_op<F: Fn(i64, i64) -> i64>(a: &Value, b: &Value, f: F) -> Result<Value> {
    let num_a = a.as_number()?;
    let num_b = b.as_number()?;
    let num_result = f(num_a as i64, num_b as i64) as f64;
    Ok(num_result.into())
}

#[allow(clippy::unnecessary_wraps)]
fn bool_op<F: Fn(&Value, &Value) -> bool>(a: &Value, b: &Value, f: F) -> Result<Value> {
    let bool_result = f(a, b);
    Ok(bool_result.into())
}
