use crate::{Result, Symbol, Value};

#[allow(clippy::module_name_repetitions)]
pub fn check_type(type_: Symbol, expected_type: Symbol) -> Result<()> {
    if type_ != expected_type {
        return Err(format!("expected {}, got {}", expected_type, type_));
    }

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn check_count(type_: Symbol, values: &[Value], expected_count: usize) -> Result<()> {
    let count = values.len();
    if count != expected_count {
        return Err(format!(
            "expected {} with {} values, got {}",
            type_, expected_count, count
        ));
    }

    Ok(())
}
